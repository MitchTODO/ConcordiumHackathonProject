import React, { useState, useEffect } from "react";

import '../Dapp.css';
import './ConnectId.css';

import SideBar from "./SideBar";
import OverView from "./OverView";


function WillOverView(props) {

    const [subState,setSubState] = useState(0);
    const [textState,setTextState] = useState("Checking for existing documents...");
    const [loading,setLoading] = useState(false);

    const [willCount,setWillCount] = useState(null);
    const [wills,setWills] = useState([]);
    const [activeWill,setActiveWill] = useState(null);


    useEffect(() => {
        // set default state and show progress
        setSubState(0)
        setLoading(true)
        
        // fetch existing documents 
        if (props.account != null) {
            getWillCount(props.account) 
        }
    },[
        props.account,
        props.contractService
    ])


    
    // Get user will count
    const getWillCount = async (accountAddress) => {
        props.contractService.willCount(accountAddress,0)
            .then(willCount => {
                getWill(accountAddress,willCount)

                if (willCount == 0){
                    //setTextState("No exisiting documents.")
                    setLoading(false)
                }else{
                    //setTextState(`Fetching ${willCount} exisiting documents...`)
                }
            })
            .catch((error) => {
                console.log(error);
            })
    }


    // Get will 
    const getWill = async (accountAddress,amount) => {
        let willArray = [];
        for(let w = 0; w <= amount - 1; w++) {
            // call get wills methods from contract service class
            props.contractService.getWills(accountAddress,accountAddress,w)
            .then(willObject => {
                if (willObject.hasOwnProperty("Some")) {
                    willObject.Some[0].id = w;
                    willArray.push(willObject.Some[0])
                    if (w == amount - 1) {
                        setWillCount(amount)
                        setLoading(false)
                        setWills(willArray)
                        getActiveWill(accountAddress)
                    }
                }
            })
            .catch((error) => {
                console.log(error);
            })
        }
    }


    const getActiveWill = async (accountAddress) => {
        props.contractService.activeWill(0,accountAddress)
        .then(willObject => {
            if (willObject.hasOwnProperty("Some")) {
                setActiveWill(willObject.Some[0])
            }
        })
        .catch((error) => {
            console.log(error);
        })
    }
    
    return (
        <div className="d-flex justify-content-around match_window">
            <SideBar 
                subState = {setSubState}
            />
            <OverView
                loading = {loading}
                subState = {subState}
                willCount = {willCount}
                wills = {wills}
            />
        </div>

    )

}

export default WillOverView;