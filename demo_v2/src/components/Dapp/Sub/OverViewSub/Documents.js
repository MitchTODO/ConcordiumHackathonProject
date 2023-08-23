import React, { useState, useEffect } from "react";

import "./sub.css"
import File from "./SubDocument/File";


function Documents(props) {

    const [files,setFiles] = useState([]);

    useEffect(() => {
        setFiles([])
        let tempFiles = [];
        for(let i = 0; i < props.wills.length; i++) {
            tempFiles.push(<File key = {i} will = {props.wills[i]}/>);
        }
        setFiles(tempFiles);
        
    },[props.wills])

    return (
        <div className="p-4">
            
            <div className="view-margin">
                <h3>Upload your Will for E-Sealing and notarization.</h3> 
            </div>

            <div className="d-flex justify-content-around view-margin ">
                <div>
                    <b>Authenticity</b>
                    <p>
                        Authenticity
                    </p>
                </div>

                <div>
                    <b>Immutablility</b>
                    <p>
                        Immutablility
                    </p>
                </div>

                <div>
                    <b>Discovery</b>
                    <p>
                        Discovery
                    </p>
                </div>
            </div>

           <div className="d-flex view-margin">
                <button className="btn btn-light p-3"><b>Upload Document</b></button>
                <button className="btn ml-5"><b>Learn more</b></button>
           </div>

           <div className="view-margin">
                <div className="d-flex">
                    <h5><b>All files </b></h5>
                    <p>{props.willCount}</p>
                </div>

                <div className="w-100 file-view ">

                    {props.loading && 
                        <div className="spinner-border" role="status">
                        </div>
                    }

                    <div className="d-flex justify-content-between header p-1">
                        <p>Name</p>     
                        <p>Status</p>
                        <p>Client</p>
                        <p>Last Updated</p>
                        <p></p>
                    </div>

                    <div id = "files" className="files">
                        {files}
                    </div>
                </div>
           </div>
        </div>
    )
}
export default Documents;