import React, { useState, useEffect } from "react";
import "../Home.css"

function HowItWorks(props) {

    const [info,setInfo] = useState("");
    const [id,setId] = useState("1");
    

    useEffect(() => {
        setInfo(Info.Create)
    },[
        
    ])

    const Info = {
        Create: "Will and identiy creation",
        Upload: "Will encryption and uploading",
        Notarize: "Will Notarized of a uploaded will",
        Discover: "Judge court order for a Will linked to your idenitiy."
    }


    const willSteps = (e) => {
        setId(e.target.id)
        switch(e.target.id) {
            case "1":
                setInfo(Info.Create)
                
                break;
            case "2":
                setInfo(Info.Upload)
                break;
            case "3":
                setInfo(Info.Notarize)
                break;
            case "4":
                setInfo(Info.Discover)
                break;
        }
    }

    return (
        <div className=" d-flex text-right flex-column seperator-color ">
            <div className=" d-flex justify-content-center m-5">
                <div className=" d-flex flex-column rounded justify-content-center w-75">
                    <h3>{info}</h3>
                </div>
                <div className="d-flex" >
                    <div className="vr"></div>
                </div>
                <div className="d-flex flex-column w-25 align-middle m-0 ">
                    <h2 className="m-3 text-right "> How It Works </h2>
              
                    <button id = "1" onClick={willSteps} className={`m-5 btn ${id == "1" ? "btn-primary": "btn-dark"}`}>1. Create</button>
                    <button id = "2" onClick={willSteps} className={`m-5 btn ${id == "2" ? "btn-primary": "btn-dark"}`}>2. Upload</button>
                    <button id = "3" onClick={willSteps} className={`m-5 btn ${id == "3" ? "btn-primary": "btn-dark"}`}>3. Notarize</button>
                    <button id = "4" onClick={willSteps} className={`m-5 btn ${id == "4" ? "btn-primary": "btn-dark"}`}>4. Discover</button>
                </div>
            </div>
        </div>
    )
}
export default HowItWorks;