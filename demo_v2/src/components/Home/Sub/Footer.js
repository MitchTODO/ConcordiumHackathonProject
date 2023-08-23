import React, { useState, useEffect } from "react";

import "../Home.css"

function Footer(props) {
  
    useEffect(() => {
      // use enum for user state
      // set user view
      //userState()
    },[
      
    ])
  
    return (
        <div className="container">

            <footer className="d-flex flex-wrap justify-content-between align-items-center py-3 my-4 border-top">
                <div className="col-md-4 d-flex align-items-center">
                <a href="/" className="mb-3 me-2 mb-md-0 text-muted text-decoration-none lh-1">
                    
                </a>
                <span className="mb-3 mb-md-0 text-muted">© 2022 E-Wills</span>
                </div>

                <ul className="nav col-md-4 justify-content-end list-unstyled d-flex">
                    <li className="nav-item ">
                        <button type="button" onClick ={console.log("Twitter")} className="btn m-2">
                            <img onClick={console.log("here")}  height={20} src="twitterLogoBlack.svg"  />
                        </button>
                        <button type="button" onClick ={console.log("Twitter")} className="btn m-2">
                            <img onClick={console.log("here")}  height={20} src="github-mark.png"  />
                        </button>
                    </li>
                </ul>
            </footer>

            </div>
    )
  }
export default Footer;