import React, { useState, useEffect } from "react";
//import CreateWill from "./CreateWill";

import './ConnectId.css';

function WillView(props) {

    const [willFile, setWillFile] = useState("");

    useEffect(() => {
        if (props.will != null) {
            getWillFile(props.will.will_file,props.will)
        }
    },[props.will])

    // Get will file from url
    // Parameters: `Will` will object 
    function getWillFile(willFile,will) {
        //will_file = willFile
        fetch(willFile, {mode: 'cors'})
            .then((response) => response.body)
            .then((rb) => {
                const reader = rb.getReader();
                return new ReadableStream({
                    start(controller) {
                        // handle pushed data chunk
                        function push() {
                        // "done" is a Boolean and value a "Uint8Array"
                        reader.read().then(({ done, value }) => {
                            // If there is no more data to read
                            if (done) {
                                controller.close();                            
                                return;
                            }
                            // Get the data and send it to the browser via the controller
                            controller.enqueue(value);
                            
                            push();
                        });
                        }
                        push();
                    },
                });
            })
            .then((stream) =>
                // Respond with our stream
                new Response(stream, { headers: { "Content-Type": "text/html" } }).text()
            )
            .then((result) => {
                // verify will file 'result' is string 'will' will object
                setWillFile(result)
            })
            .catch((error) => {
                alert(error);
        });
    }

    return (
        <div className="card fix-will m-2">
            {props.will != null ?
                <div className="card-body">
                    <h5 className="card-title black-text">Will {props.will.id}</h5>
                    <p className="card-text overflow p-height black-text text-small indent">{willFile}</p>
                    <p href="#" className="card-link black-text">Notarized : {String(props.will.is_notarized)}</p>
                    <h6 className="card-subtitle mb-2 small-font text-muted">{props.will.will_hash}</h6>
                </div>
                :
                <div className="card-body">
                    <div className="spinner-border" role="status">
                    <span className="sr-only"></span>
                    </div>
                </div>
            }
        </div>
    )

}
export default WillView;