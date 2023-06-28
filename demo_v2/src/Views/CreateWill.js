import React, { useState, useEffect } from "react";

import { sha256 } from "../WillUtils";

import {
    IdStatementBuilder,
    verifyIdstatement,
    AttributesKeys,
    getIdProof,
} from '@concordium/web-sdk';
import { Buffer } from 'buffer';
import { create as ipfsHttpClient } from "ipfs-http-client";
import ContractServices from "../ContractServices";

const projectId = process.env.REACT_APP_PROJECT_ID;
const projectSecretKey = process.env.REACT_APP_PROJECT_KEY;
const authorization = "Basic " + Buffer.from(projectId + ":" + projectSecretKey).toString('base64');

function CreateWill(props) {

    const [userView, setUserView] = useState(null);
    const [willHash,setWillHash] = useState("");
    const [willBuffer,setWillBuffer] = useState(null);
    const [notary,setNotary] = useState("4T3hdeNz5nZ7BGFFdGZFZ5SWaiFKkBLPLEWpdCkD3g6KBHxXUh");

    const ipfs = ipfsHttpClient({
        url: "https://ipfs.infura.io:5001/api/v0",
        headers: {
          authorization,
        },
      });

  useEffect(() => {

  },[props.willCount,props.account])

  const importWillFile = async (e) => {

    let file = e.target.files[0];
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
    reader.readAsArrayBuffer(file)
  }

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

  const mintWill = async () => {   
    // Check inputs
    // fileBuffer | fileHash | sender | Notary
    if (willBuffer == null || willHash == "" || notary == "") {
        console.log("check inputs");
    }

    // Upload will to ipfs
    // Use proof id's to encrypt the will 

    //console.log(willBuffer);
    const result = await ipfs.add(willBuffer);

    // Write will to chain 
    props.contractServices.mint(props.account,result.path,willHash,notary)
    .then(txHash => {
        console.log(txHash);
    })
    .catch((error) => {
        console.log(error);
    })
  }

  const updateInputValue = (evt) => {
    const val = evt.target.value;   
    setNotary(val);
  }

  return (
    <div className="popUp ">
        <div className="card text text-center " >
            <div className="card-body">
                <h5 className="card-title">Upload New Will</h5>
              
                <p className="card-text">Select your will to be upload and linked to your digital identity on the Concordium blockchain.</p>
       
                <form>  
                    <div className="form-group p-4 ">
                        <label htmlFor="exampleFormControlFile1">Select Your will file</label>
                        <input type="file" className="form-control-file p-2"  id="exampleFormControlFile1" onChange={importWillFile}/>
                        <input type="password" className="form-control" disabled id="exampleInputPassword1" placeholder={willHash}/>
                    </div>
                    
                    <div className="form-group ">
                        <label className="card-subtitle text-muted" htmlFor="exampleInputEmail1">Notary Address Identity</label>
                        <input type="text" className="form-control " id="exampleInputEmail1" aria-describedby="emailHelp" value ={notary} onChange={evt => updateInputValue(evt)} placeholder="3NoqELh96cEhE..."/>
                        <small>Identity that is allows to notarize your will.</small>
                    </div>
                    <button type="button" className="btn btn-success m-2" onClick={mintWill} >Upload</button>
                </form>
            </div>
        </div>
    </div>
    )
}

export default CreateWill;
