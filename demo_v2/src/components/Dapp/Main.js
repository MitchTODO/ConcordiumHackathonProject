import React, { useState, useEffect } from "react";

import PreviousWills from "./PreviousWills";
import ActiveWill from "./Sub/ActiveWill";
import '../App.css';
import Notary from "./Notary";

function Main(props) {

  const [userView, setUserView] = useState(null);
  const [showNotary,setShowNotary] = useState(false);

  useEffect(() => {
    // use enum for user state
    // set user view
    userState()
  },[
    props.willCount,
    props.account,
    props.contractServices,
    props.wills,
    props.activeWill,
  ])

  function toggleNotaryView() {
    if (showNotary) {
      setShowNotary(false);
    }else{
      setShowNotary(true);
    }
  }

  function userState() { 
    const willCount = props.willCount;
      let view;
      if(willCount == null){
        setUserView(<p className='text-center light-text'>Connect your Concordium Wallet to get started.</p>);
      }else{
        setUserView(
          <div>
              <PreviousWills
                willCount = {props.willCount}
                contractServices = {props.contractServices}
                wills = {props.wills}
              />

              <ActiveWill
                activeWill = {props.activeWill}
              />
          </div>
          )
      }
    return view
  }

  return (
    <div>
      {userView}
    </div>
  )

}
export default Main;
