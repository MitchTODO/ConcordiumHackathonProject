import React, { useState, useEffect } from "react";
import CreateWill from "./CreateWill";
import '../App.css';


function WillView(props) {

    const [willFile, setWillFile] = useState("");

    useEffect(() => {
        if (props.will != null) {
            let willFile = "https://cloudflare-ipfs.com/ipfs/"+props.will.will_file
            getWillFile(willFile,props.will)
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
                //verifiyEseal(result,will);
                //console.log(result);
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
                    <h5 className="card-title">Will {props.willId}</h5>
                    <p className="card-text overflow p-height ">{willFile}</p>
                    <p href="#" className="card-link">Notarized : {String(props.will.is_notarized)}</p>
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