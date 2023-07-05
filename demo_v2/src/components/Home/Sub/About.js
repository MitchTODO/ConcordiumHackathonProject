import React, { useState, useEffect } from "react";

import "../Home.css"


function About(props) {

  useEffect(() => {
    // use enum for user state
    // set user view
    //userState()
  },[
    
  ])



  return (
    <div className=" d-flex black justify-content-center">
        <div className=" d-flex flex-column  box-height trans-white align-middle w-50 text-center align-middle m-0 ">
          <h3 className="text-dark mt-5">
              Next Generation of electronic Wills 
          </h3>
            <h3 className="text-dark mt-5">
                Notarization,
                <br/>
                Security,
                <br/>
                Immutability,
                <br/>
                Discovery
                <br/>
               
            </h3>
            <h3 className="text-dark mt-5"> Powered by the Concordium Blockchain.</h3>
        </div>
    </div>
  )
}
export default About;