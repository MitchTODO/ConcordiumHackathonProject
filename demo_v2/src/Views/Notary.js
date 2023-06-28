import React, { useState, useEffect } from "react";
import '../App.css';
import { toBuffer } from "@concordium/web-sdk";
import { sha256 } from "../WillUtils";


function Notary(props) {

    const [testator, setTestator] = useState("");
    const [willId,setWillId] = useState("");

    const [isLoading,setIsLoading] = useState(false);
    const [uploading,setUploading] = useState(false);

    const [isNotaryForWill,setNotaryForWill] = useState(true);
    const [isNotarized,setIsNotarized] = useState(null);

    const [timestamp,setTimeStamp] = useState("");
    const [witness,setWitness] = useState("");

    const [will,setWill] = useState(null);
    const [willFile,setWillFile] = useState(null);
    const [willHash,setWillHash] = useState("");
    const [witnessAddress,setWitnessAddress] = useState("");

    useEffect(() => {
        if(will != null) {
            if (will.notary == props.account) {
                setNotaryForWill(true)
            }else{
                setNotaryForWill(false)
            }
        }
    },[props.contractService,props.account])

    // Get testator wills 
    const getWills = async () => {
        setIsLoading(true)
        
        props.contractService.getWills(props.account,testator,parseInt(willId))
        .then(willObject => {
        setIsLoading(false);
          if (willObject.hasOwnProperty("Some")) {
            
            const will = willObject.Some[0]
            const willFile = "https://cloudflare-ipfs.com/ipfs/"+will.will_file
            getWillFile(willFile,will)
            
            setWill(will);
            setIsNotarized(will.is_notarized);


            if (will.notary == props.account) {
                
                setNotaryForWill(true)
            }else{
                setNotaryForWill(false)
            }
            // check eseal 
            if (will.e_seal.timestamp.hasOwnProperty("Some")) {
                setTimeStamp(will.e_seal.timestamp.Some[0]);
                setWitness(will.e_seal.witness.Some[0]);
            }
            //setNotaryForWill(true)
            // Check notary address
            //checkNotaryForWill()       
          }else{
            // No will for input params 
          }
        })
        .catch((error) => {
            setIsLoading(false);
            console.log(error);
        }) 
    }

    const updateInputWitness = (evt) => {
        const val = evt.target.value;
        setWitnessAddress(val);
    }

    const updateInputTestator = (evt) => {
        const val = evt.target.value;   
        setTestator(val);
    }
    const updateInputWillId = (evt) => {
        const val = evt.target.value;   
        setWillId(val);
    }

    function notarize() {
        setUploading(true)
        props.contractService.notarize(props.account,willHash,willId,testator,witnessAddress)
        .then(txHash => {
            console.log(txHash);
            setUploading(false)
        })
        .catch((error) => {
            console.log(error);
            setUploading(false)
        })
    }

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
        <div className="m-5">
            <div className="container bg-secondary rounded mt-3 ">
                <h5 className="card-title text-center m4 text-light">Show Notary View</h5>
                <p className='text-center light-text'>Will Lookup</p>
                <form>  
                    <div className="form-group text-left ">
                        <label className="card-subtitle text-muted" htmlFor="exampleInputEmail1">Testator Address Identity</label>
                        <input type="text" className="form-control " id="exampleInputEmai11"  value ={testator} onChange={evt => updateInputTestator(evt)} placeholder="3NoqELh96cEhE..."/>
                        <label className="card-subtitle text-muted" htmlFor="exampleInputEmail1">Will Id</label>
                        <input type="number" className="form-control " id="exampleInputEmail2"  value ={willId} onChange={evt => updateInputWillId(evt)} placeholder="1"/>
                    </div>
                </form>
                <div className="text-center">
                    { !isLoading ?
                        <button type="button" className="btn btn-success m-2 justify-content-center text-center" onClick={getWills} >Go</button>
                    :
                        <div className="spinner-border text-light m-4 " role="status">
                            <span className="sr-only"></span>
                        </div>
                    }
                </div>
            </div>

            { will != null &&
                <div className="container bg-secondary rounded m-3 ">
                    <h5 className="card-title text-center text-light">Testator Will</h5>
                    <h6 className="card-title text-center text-light">{testator}</h6>
                    <div className="container bg-white rounded mt-3 p-4">
                        <p className="indent">{willFile}</p>
                    </div>

                    <div className="mt-3">
                        <label className="card-subtitle text-muted" htmlFor="exampleInputEmail1">Witness Address</label>
                        <input type="text" className="form-control " id="exampleInputEmai11"  value = {witnessAddress} onChange={evt => updateInputWitness(evt)} placeholder="3NoqELh96cEhE..."/>
                        <label className="card-subtitle text-muted mt-3" htmlFor="exampleInputEmail1">Hash</label>
                        <input type="text" className="form-control" disabled value = {willHash}/>
                    </div>
                    { !uploading ?
                        <div className="text-center m-3 p-3">
                            { !isNotarized ?
                                <div >
                                { isNotaryForWill ?
                                    <button type="button" className="btn btn-success m-2 justify-content-center text-center" onClick={notarize} >Notarize</button>
                                    :
                                    <div>
                                        <h5>Status: You are not active notary for this will.</h5>
                                        <button type="button" className="btn btn-success m-2 justify-content-center text-center" disabled >Notarize</button>
                                    </div>
                                }
                                </div>
                                
                            :
                            
                            <div>
                                    <div class="d-flex justify-content-around">
                                        <h6 className="card-title text-center text-light">Status: Will notarized.</h6>
                                    </div>
                                    
                                    <h6 className="card-title text-center text-light pt-4">Notary</h6>
                                    <p className="card-title text-center text-light">{will.notary}</p>

                                    <h6 className="card-title text-center text-light pt-4">Eseal</h6>
                                    <small>Timestamp</small>
                                    <p className="card-title text-center text-light">{timestamp}</p>
                                    <small>Witness</small>
                                    <p className="card-title text-center text-light">{witness}</p>
                                </div>

                        }
                        </div>

                        
                    :
                        <div className="spinner-border text-light m-4 " role="status">
                            <span className="sr-only"></span>
                        </div>
                    }
                   
                </div>
            }


        </div>


        

    )

}
export default Notary;