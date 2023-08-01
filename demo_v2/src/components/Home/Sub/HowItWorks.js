import React, { useState, useEffect } from "react";
import { Router, useNavigate } from "react-router-dom";
import "../Home.css"


function HowItWorks(props) {

    const [info,setInfo] = useState("");
    const [id,setId] = useState("1");
    

    useEffect(() => {
        setInfo(Info.Create)
    },[
        
    ])

    const navigate = useNavigate();

    function toApp() {
      navigate("/app")
    }

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
        <div className=" d-flex flex-column seperator-color p-5 ">
            <h3 className="m-3 text-dark bold right-text">
                How It Works 
              </h3>
            <div className=" d-flex justify-content-center ">
                <div className=" d-flex flex-column rounded justify-content-center w-75">
                    <h3>{info}</h3>
                </div>
                <div className="d-flex" >
                    <div className="vr"></div>
                </div>
                <div className="d-flex flex-column w-25 align-middle m-5 ">
              
                    <button id = "1" onClick={willSteps} className={`btn ${id == "1" ? "btn-primary blue-button": "btn-dark"}`}>1. Create</button>
                    <button id = "2" onClick={willSteps} className={`btn ${id == "2" ? "btn-primary blue-button": "btn-dark"}`}>2. Upload</button>
                    <button id = "3" onClick={willSteps} className={`btn ${id == "3" ? "btn-primary blue-button": "btn-dark"}`}>3. Notarize</button>
                    <button id = "4" onClick={willSteps} className={`btn ${id == "4" ? "btn-primary blue-button": "btn-dark"}`}>4. Discover</button>
                </div>
          
                
            </div>
            <button type="button" onClick ={toApp} className="btn btn-primary blue-button text-dark m-2 roboto-text">Try The Demo</button>
        </div>
    )
}
export default HowItWorks;