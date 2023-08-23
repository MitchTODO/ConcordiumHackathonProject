

import React, { useState, useEffect } from "react";

import './File.css';


function File(props) {

    const [name,setName] = useState("");
    const [status,setStatus] = useState("Draft");
    const [id,setId] = useState("");

    useEffect(() => {
        if (props.will != undefined) {
            setId(props.will.id)
        }
    },[])


    const goToFile = () => {
        
    }


    return (
        <div className="file p-1" onClick={goToFile()}>
                <div className="d-flex justify-content-between p-1">
                    <p>{id}</p>     
                    <p>{status}</p>
                    <p>None</p>
                    <p>10:36am</p>
                    <button className="btn ">...</button>
                </div>
        </div>
    )

}
export default File;