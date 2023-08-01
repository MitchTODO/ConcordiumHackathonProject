import React, { useState, useEffect } from "react";

import "../Home.css"

function MainPage(props) {
  
    useEffect(() => {
      // use enum for user state
      // set user view
      //userState()
    },[
      
    ])
  
    return (
      <div className="d-flex blueBack justify-content-around main-height p-5 " >
        <div className=" auto-margin w-50 right-margin">
          <h3 className="bold no-white">
                Experience the power of E-Wills
            </h3>
            <p>
              E-Wills, the cutting-edge solution revolutionizing the world of estate planning and asset distribution. Powered by the innovative Concordium Blockchain.
            </p>
            <button type="button" class="btn btn-light blue-text">Learn More</button>
        </div>
        <div className="w-50">
          <img className="card-img-top" src="..." alt="image cap"/>
        </div>
      </div>
    )
  }


export default MainPage;