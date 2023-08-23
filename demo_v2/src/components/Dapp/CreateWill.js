import React, { useState, useEffect } from "react";

import { sha256 } from "./Utils/WillUtils";

import {
    IdStatementBuilder,
    verifyIdstatement,
    AttributesKeys,
    getIdProof,
} from '@concordium/web-sdk';
import { Buffer } from 'buffer';
import { create as ipfsHttpClient } from "ipfs-http-client";

import "./Dapp.css"
import "./Sub/ConnectId.css"

import {getTxStatus} from "./Utils/WillUtils";

const projectId = process.env.REACT_APP_PROJECT_ID;
const projectSecretKey = process.env.REACT_APP_PROJECT_KEY;
const authorization = "Basic " + Buffer.from(projectId + ":" + projectSecretKey).toString('base64');

function CreateWill(props) {

    //const [notaryTitle,setNotaryTitle] = useState("");

    const [subState,setSubState] = useState(null);
    const [willHash,setWillHash] = useState("");
    const [willBuffer,setWillBuffer] = useState(null);
    const [willUri,setWillUri] = useState("");
    const [willfile,setWillFile] = useState("");

    const [notary,setNotary] = useState("4T3hdeNz5nZ7BGFFdGZFZ5SWaiFKkBLPLEWpdCkD3g6KBHxXUh");
   
    const ipfs = ipfsHttpClient({
        url: "https://ipfs.infura.io:5001/api/v0",
        headers: {
          authorization,
        },
      });

  useEffect(() => {
    setSubState(0)
  },[props.willCount,props.account,props.contractService])

  const importWillFile = async (e) => {
    let file = e.target.files[0];
    
    setSubState(1)

    setWillUri(file.name)
    getFileText(file)
    getFileBuffer(file)
  }


    const getFileText = async (file) => {
        var reader = new FileReader();
        reader.onload = function(event) {
            setWillFile(event.target.result);
        }
        reader.readAsText(file, "UTF-8");
    }

    const getFileBuffer = async (file) => {
        var reader = new FileReader();
        reader.onload = function(event) {
            let willBuffer = event.target.result;
            setWillBuffer(willBuffer)
            sha256(willBuffer)
            .then(hash => {
                setWillHash(hash);
                // Encrypte file here
            });
        }
        reader.readAsArrayBuffer(file);
    }

  const clear = () => {
    setWillUri("")
    setWillFile("")
  }


  /*
  const encryptWill = async () => {
    // Use id proofs and account to encrypt will
    const statementBuilder = new IdStatementBuilder();
    statementBuilder.revealAttribute(AttributesKeys.firstName);
    statementBuilder.revealAttribute(AttributesKeys.lastName);
    statementBuilder.revealAttribute(AttributesKeys.sex);
    statementBuilder.revealAttribute(AttributesKeys.dob);
    statementBuilder.revealAttribute(AttributesKeys.countryOfResidence);
    statementBuilder.revealAttribute(AttributesKeys.nationality);

    const statement = statementBuilder.getStatement();
    verifyIdstatement(statement);

    //const proof = getIdProof({
    //    statement,
    //})
    //console.log(proof);
  }
  */

    const mintWill = async () => { 
        setSubState(3)

        // Check inputs
        // fileBuffer | fileHash | sender | Notary
        if (willBuffer == null || willHash == "" || notary == "") {
            console.log("check inputs");
        }
        console.log("mint will");
        // Upload will to ipfs
        // Use proof id's to encrypt the will 
        // upload image to ipfs
        const result = await ipfs.add(willBuffer);

        console.log(result.path);
        
        let nftMetaData = {
            "name": "E-Will - Proof Of Ownership",
            "unique": true,
            "description": "This NFT collectable show proof of ownership of a eletronic will and testament (E-Will). ",
            "thumbnail": { "url": "https://cloudflare-ipfs.com/ipfs/QmfP8x7vkSZPUfNGNk4wcjzKBpVAcABVcugnfmJgFrLXJz" },
            "display": { "url": "https://cloudflare-ipfs.com/ipfs/QmeERoCjBi61LcJ11By8poArgzdQRxXChsGw3vjUtdCYgB" },
        }
        

        const nft_url = await ipfs.add(JSON.stringify(nftMetaData))
        
        console.log(nft_url.path);
        // Write will to chain 
        props.contractService.mint(props.account,result.path,willHash,notary)
        .then(txHash => {
            // update state 
            setSubState(4)
            // get tx status
            getTxStatus(props.client,txHash).then(result => {
                // tx was successful 
                if(result) {
                    //alert("Transaction Successful")
                    props.scheduleNotary()
                }else { // failed 
                    alert("Transaction Failed")
                }
            })
        })
        .catch((error) => {
            console.log(error);
        })
        
    }

    const selectNotary = () => {
        setSubState(2)
    }

    const updateInputValue = (evt) => {
        const val = evt.target.value;   
        setNotary(val);
      }
  
    // props.scheduleNotary
    const viewSteps = () => {
        switch(subState) {
            
            case 0:
                return(
                    <div>
                        <h2 className="text-center">Create or Upload Will file</h2>
                        <div className="d-flex justify-content-center">
                            <label htmlFor="files"  type="button" className="btn btn-light btn-lg dark-text m-5" >Upload Will</label>
                            <input id="files" className="invisibility" type="file" onChange={importWillFile}/>
                            <button type="button" className="btn btn-light disabled btn-lg dark-text m-5" >Create Will</button>
                        </div>
                    </div>
                )
            case 1:
                return(
                    <div>
                        <h2 className="text-center">Create or Upload Will file</h2>
                        <div className="d-flex justify-content-around margin-top">
                            <button className="btn btn-dark " onClick={clear}>Change File</button>
                            <button className="btn btn-light dark-text margin-bottom" onClick={selectNotary}>Select Your Notary</button>
                        </div>
                        <div className=" bg-light rounded will-margin">
                            <p className="text-dark p-5 will-margin indent black-text left-padding right-padding will-spacing ">
                                {willfile}
                            </p>
                            <p className="text-dark text-center"><b>{willHash}</b></p>
                            <p className="text-dark text-center"><b>{willUri}</b></p>
                        </div>
                    </div>
                )
            case 2:
                return(
                    <div className="justify-content-center">
                        <h2 className="text-center">Select Your Notary</h2>
                        <div className="width-text ">
                                <label className="card-subtitle light-text text-center" htmlFor="exampleInputEmail1">Notary Address Identity</label>
                                <input type="text" className="form-control " id="exampleInputEmail1"  value ={notary} onChange={evt => updateInputValue(evt)} placeholder="3NoqELh96cEhE..."/>
                                <button type="button" className="btn btn-light dark-text mt-3" onClick={mintWill}> Save will to Blockchain</button>
                        </div>
                    </div>
                )
            case 3:
                return(
                    <div>
                        <h2 className="text-center">Saving</h2>
                        <div className="d-flex justify-content-center">
                            <div className="spinner-border text-light" role="status">
                                <span className="sr-only"></span>
                            </div>
                        </div>
                    </div>
                )
            case 4:
                return(
                    <div>
                        <h2 className="text-center">Validating Transaction</h2>
                        <div className="d-flex justify-content-center">
                            <div className="spinner-border text-light" role="status">
                                <span className="sr-only"></span>
                            </div>
                        </div>
                    </div>
                )

        }
    }

  return (
        <div className="popUp">
            {viewSteps()}
        </div>
    )
}

export default CreateWill;
