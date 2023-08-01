import React, { useState, useEffect } from "react";

import "../Home.css"
import Card from "./Card";


function About(props) {

  useEffect(() => {
    // use enum for user state
    // set user view
    //userState()
  },[
    
  ])

  return (
    <div className=" main-height">
        <div className="d-flex justify-content-between p-5">
          <div className=" auto-margin w-50 ">
            <h3 className="text-dark bold">
                  About
              </h3>
          </div>
          <div className=" auto-margin w-50">
            <p className="text-dark">
                At E-Wills, we are committed to safeguarding your assets and ensuring that your final wishes are honored with utmost transparency and security. Our platform leverages the transformative potential of blockchain technology to deliver an unbreakable foundation for your wills and trusts.
              </p>
          </div>
        </div>

        <div className="d-flex justify-content-between p-5">
            <Card 
              title = {"The Power of Authenticity"}
            />
            <Card
              title = {"Ownership Redefined"}
            />
            <Card
              title = {"Secure Your Legacy"}
            />
        </div>

        <div className="side-auto-margin p-5">
          
            <h4 className="text-dark bold text-center">
                  Discovery
              </h4>
   
            <p className="text-dark text-center">
              Will Discovery 
            </p>
   
        </div>

    </div>
  )
}
export default About;