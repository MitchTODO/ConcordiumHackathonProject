import React, { useState, useEffect } from "react";

import "../Home.css"

function Card(props) {
  
    useEffect(() => {
      // use enum for user state
      // set user view
      //userState()
    },[
      
    ])
  
    return (
        <div className="card w-25" >
            <img className="card-img-top" src="..." alt="Card image cap"/>
            <div className="card-body">
                <h5 className="card-title text-dark">{props.title}</h5>
            </div>
        </div>
    )
  }
export default Card;