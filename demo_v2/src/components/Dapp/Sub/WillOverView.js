import React, { useState, useEffect } from "react";

import '../Dapp.css';
import './ConnectId.css';

import PreviousWills from "./PreviousWills";
import ActiveWill from "./ActiveWill";
import CreateWill from "../CreateWill";
import Notarize from "./Notarize";


function WillOverView(props) {

    const [subState,setSubState] = useState(null);
    const [textState,setTextState] = useState("Checking for existing wills...");
    const [loading,setLoading] = useState(false);

    const [willCount,setWillCount] = useState(null);
    const [wills,setWills] = useState([]);
    const [activeWill,setActiveWill] = useState(null);

    useEffect(() => {
        setSubState(0)
        setLoading(true)
        // fetch existing wills 
        if (props.account != null) {
            getWillCount(props.account) 
        }
    },[
        props.account,
        props.contractService
    ])

    const createWillAction = () => {
        setSubState(2)
    }

    const scheduleNotary = () => {
        setSubState(1)
        getWillCount(props.account)
    }

    // get user will count
    const getWillCount = async (accountAddress) => {
        props.contractService.willCount(accountAddress,0)
            .then(willCount => {
                setWillCount(willCount)
                getWill(accountAddress,willCount)
                if (willCount == 0){
                    setTextState("No exisiting wills...")
                }else{
                    setTextState(`Fetching ${willCount} exisiting wills`)
                }
            })
            .catch((error) => {
                console.log(error);
            })
    }


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
                        setLoading(false)
                        setWills(willArray)
                        setSubState(1)
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

    const viewSteps = () => {
        switch(subState) {
            case 0:
                return(
                    <div className="d-flex justify-content-center text-center">
                        <div>
                            <h5>{textState}</h5>
                            <div className="spinner-border text-light" role="status">
                                <span className="sr-only"></span>
                            </div>
                        </div>
                    </div>
                )
            case 1:
                return(
                    <div className="mt-5">
                        <PreviousWills
                            createWillAction = {createWillAction}
                            willCount = {willCount}
                            contractServices = {props.contractService}
                            wills = {wills}
                        />
                        <ActiveWill
                            activeWill = {activeWill}
                        />
                    </div>
                )
            case 2:
                return(
                    <div className="mt-5">
                        <CreateWill
                            client = {props.client}
                            scheduleNotary = {scheduleNotary}
                            contractService = {props.contractService}
                            account = {props.account}
                        />
                    </div>
                )
            case 3:
                return(
                    <div className="mt-5">
                        <Notarize
                            client = {props.client}
                            contractService = {props.contractService}
                            account = {props.account}
                        />
                    </div>
                )
        }
    }

    return (
        <div>
            {viewSteps()}
        </div>
       
    )

}

export default WillOverView;