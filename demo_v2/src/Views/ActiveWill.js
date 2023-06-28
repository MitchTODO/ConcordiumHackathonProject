import React, { useState, useEffect } from "react";

import '../App.css';
import { toBuffer } from "@concordium/web-sdk";
import { sha256 } from "../WillUtils";


function ActiveWill(props) {

    const [will,setWill] = useState(null);
    const [willFile,setWillFile] = useState("");
    const [willHash,setWillHash] = useState("");

    const [isLoading,setIsLoading] = useState(true);

    
    useEffect(() => {
        // use enum for user state
        // set user view
        if(props.activeWill != null) {
            const will = props.activeWill
            setWill(will)
            const willFile = "https://cloudflare-ipfs.com/ipfs/"+will.will_file
            getWillFile(willFile,will)
        }
    },[props.activeWill])

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
                
                setWillFile(result)
                setIsLoading(false)
                let buffer = toBuffer(result)
                sha256(buffer)
                .then(hash => {
                    setWillHash(hash);
                });
                
            })
            .catch((error) => {
                alert(error);
        });
    }


    return (
        <div className="container bg-secondary rounded mt-3">
            <h5 className="card-title text-center m4 text-light">Your Active Will</h5>
            { isLoading ?
                <div className="spinner-border text-light m-4 " role="status">
                    <span className="sr-only"></span>
                </div>
            :
                <div className="container bg-secondary rounded ">
                    <div className="container bg-white rounded mt-3 p-4">
                        <p className="indent">{willFile}</p>
                    </div>
                    <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">E-Seal</h5>
                        <h6 class="card-subtitle mb-2 text-success">Verified</h6>
                        <p class="card-text">E-Will is verified and protected.</p>
                        <p href="#" class="card-link">Timestamp: {will.e_seal.timestamp.Some[0]}</p>
                        <p href="#" class="card-link">Witness: {will.e_seal.witness.Some[0]}</p>
                        <p>Will Hash: {will.will_hash}</p>
                    </div>
                    </div>
                    <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">Notarized</h5>
                        <h6 class="card-subtitle mb-2 text-success">Success</h6>
                        <p class="card-text">Will has be notarized.</p>
                        <p href="#" class="card-link">Notary: {will.notary}</p>
                        <p href="#" class="card-link">Witness: {will.e_seal.witness.Some[0]}</p>
                    </div>
                    </div>
                </div>
            }
        </div>
    )
}

export default ActiveWill;