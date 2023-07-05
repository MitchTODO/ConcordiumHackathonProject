import React, { useState, useEffect } from "react";
import logo from './components/assets/logo.png';
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
    <nav className="navbar primary-color shadow">
        <img className=" p-2 navbar-brand m-2 " onClick={console.log("here")}  height={55} src={logo} />
        <div className=" p-2 bd-highlight">
          <ul className="navbar-nav d-flex flex-row">
          <li className="nav-item px-2">
              <button type="button" onClick ={console.log("about")} className="btn m-2">About</button>
            </li>
            <li className="nav-item px-2">
              <button type="button" onClick ={console.log("htw")} className="btn m-2">How It Works</button>
            </li>
            <li className="nav-item px-2">
              <button type="button" onClick ={console.log("road map")} className="btn m-2">Road Map</button>
            </li>
            <li className="nav-item px-2">
              <button type="button" onClick ={toApp} className="btn btn-dark m-2">Launch App</button>
            </li>
          </ul>
          </div>
      </nav>
    </div>

  )
}


/*
class Navi extends React.Component {

  constructor(props) {
    super(props);
      
      this.connectAccount = this.connectAccount.bind(this)
  }

  // handle navi button clicks
  async connectAccount() {
    const navigate = useNavigate();
    navigate("/app");

    //console.log("Value");
  }

  render() {
    return (


    );
  }
}
*/


export default Navi;