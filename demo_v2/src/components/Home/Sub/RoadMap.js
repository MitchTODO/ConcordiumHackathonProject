import React, { useState, useEffect } from "react";

import "../Home.css"

function RoadMap(props) {

  useEffect(() => {

  },[])

  return (
    <div className=" d-flex flex-column " >
        <span>
          <h2>Timeline</h2>
        </span>
        <div className="d-flex justify-content-center top-border">
          <div>

          <div className="timeline-box mr-5" style={{top: '100px'}}>
              <h3>Proof of concept</h3>
              <p>Create a proof of concept</p>
            </div>

            <div className="timeline-box mr-5" style={{top: '320px'}}>
              <h3>MVP</h3>
            </div>

            <div className="timeline-box mr-5" style={{top: '540px'}}>
              <h3>Funding</h3>
            </div>

            <div className="timeline-box mr-5" style={{top: '800px'}}>
              <h3>Production</h3>
            </div>

          </div>
          <div className="line-holder d-flex justify-content-center">
            <div className="line" ></div>
            
            <div>
              <div className="circle" style={{top: '100px'}}></div>
              <div className="circle" style={{top: '320px'}}></div>
              <div className="circle" style={{top: '540px'}}></div>
              <div className="circle" style={{top: '800px'}}></div>
            </div>
           
          </div>
          <div>
            <div className="timeline-box" style={{top: '100px'}}>
              <h3>Time</h3>
            </div>
            <div className="timeline-box" style={{top: '320px'}}>
              <h3>Time</h3>
            </div>
            <div className="timeline-box" style={{top: '540px'}}>
              <h3>Time</h3>
            </div>
            <div className="timeline-box" style={{top: '800px'}}>
              <h3>Time</h3>
            </div>
          </div>
        </div>  
    </div>
  )
}
export default RoadMap;