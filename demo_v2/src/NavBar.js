import React, { useState, useEffect } from "react";

import { Router, useNavigate } from "react-router-dom";
import './App.css';

function Navi(props) {

  const navigate = useNavigate();

  function toApp() {
    navigate("/app")
  }

  useEffect(() => {

  },[

  ])

  return(
    <div>
    <nav className="navbar primary-color shadown roboto-text ">
        <button type="button" onClick ={console.log("Twitter")} className="btn ">
          <img className=" navbar-brand " onClick={console.log("here")}  height={55} src="E-WillsLOGO.png" />
          <img className=" navbar-brand " onClick={console.log("here")}  height={55} src="E-WillsTEXT.JPG" />
        </button>
        
        <div className="  bd-highlight ">
          <ul className="navbar-nav d-flex flex-row">
          <li className="nav-item ">
              <button type="button" onClick ={console.log("about")} className="btn m-2 text-dark roboto-text">About</button>
            </li>
            <li className="nav-item ">
              <button type="button" onClick ={console.log("htw")} className="btn m-2 text-dark roboto-text">How It Works</button>
            </li>
            <li className="nav-item ">
              <button type="button" onClick ={console.log("road map")} className="btn m-2 text-dark roboto-text">Road Map</button>
            </li>
            <li className="nav-item ">
              <button type="button" onClick ={toApp} className="btn btn-primary blue-button m-2 roboto-text">Demo</button>
            </li>
            <li className="nav-item ">
              <button type="button" onClick ={console.log("Twitter")} className="btn m-2">
                <img onClick={console.log("here")}  height={20} src="twitterLogoBlack.svg"  />
              </button>
              <button type="button" onClick ={console.log("Twitter")} className="btn m-2">
                <img onClick={console.log("here")}  height={20} src="github-mark.png"  />
              </button>
            </li>

          </ul>
          </div>
      </nav>
    </div>

  )
}



export default Navi;