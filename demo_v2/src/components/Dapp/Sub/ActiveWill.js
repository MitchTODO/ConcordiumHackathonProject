import React, { useState, useEffect } from "react";

import { toBuffer } from "@concordium/web-sdk";
import { sha256 } from "../Utils/WillUtils";

import "./ActiveWill.css";

function ActiveWill(props) {

    const [will,setWill] = useState(null);
    const [willFile,setWillFile] = useState("");
    const [willHash,setWillHash] = useState("");

    const [timestamp,setTimestamp] = useState("");
    const [witness, setWitness] = useState("");
    const [willUrl,setWillUrl] = useState("");

    const [isLoading,setIsLoading] = useState(true);

    useEffect(() => {
        // use enum for user state
        // set user view
        if(props.activeWill != null) {
            const will = props.activeWill;

            if(will.e_seal.timestamp.hasOwnProperty("Some")) {
                setTimestamp(will.e_seal.timestamp.Some[0])
                setWitness(will.e_seal.witness.Some[0])
            }

            setWill(will);
            const willFile = "https://cloudflare-ipfs.com/ipfs/"+will.will_file;
            setWillUrl(willFile)
            getWillFile(willFile,will);
        }
    },[props.activeWill])

    // Get will file from url
    // Parameters: `Will` will object 
    function getWillFile(willFile,will) {
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
                // verifiyEseal(result,will);
                // console.log(result);
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
        <div className="container bg-secondary big-padding rounded mt-3">
            <h3 className="card-title text-left text-light">Your Active Will</h3>
            { isLoading ?
                <div className="spinner-border text-light m-4 " role="status">
                    <span className="sr-only"></span>
                </div>
                :
                <div className="container bg-secondary rounded ">
                    <div className="container bg-white rounded mt-3 p-4">
                        <p className="indent black-text left-padding right-padding will-spacing ">{willFile}</p>
                        <div className="d-flex justify-content-center">
                            <div className="justify-content-center text-center m-4">
                                <h4> <u>E-Seal Verification</u> </h4>
                                <div className="d-flex justify-content-center">
                                    <h5 className="black-text">{will.is_notarized} </h5>
                                    <h6 className="card-subtitle mb-2 text-success">Verified</h6>
                                </div>
                                <p className="black-text">{timestamp} </p>
                                <p className="black-text text-truncate fix-size">Authentication Id <br/> {will.will_hash}</p>
                            </div>
                            <div className="vr"></div>
                            <div className="justify-content-center text-center m-4">
                                <h4><u>Notarized</u></h4>
                                <div className="d-flex justify-content-center">
                                    <h6 className="card-subtitle mb-2 text-success">Success</h6>
                                </div>
                                <p className="black-text text-truncate fix-size"> Notary <br/> {will.notary}</p>
                                <p className="black-text text-truncate fix-size"> Witness <br/>  {witness}</p>
                            </div>
                        </div>
                        <div className="d-flex justify-content-center">
                            <p className="black-text text-small">{willUrl}</p>
                        </div>
                    </div>
                </div>
            }
        </div>
    )
}

export default ActiveWill;