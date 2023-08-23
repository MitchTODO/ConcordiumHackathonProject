import React, { useState, useEffect } from "react";

import '../Dapp.css';
import './ConnectId.css';


function ConnectId(props) {

    const [subView,setSubView] = useState();

    useEffect(() => {

    },[
        props.client
    ])

    const View = {
        0: <div>Connect your idenitiy</div>,
        1: <div>Create your will</div>,
        2: <div>Notary</div>
    }

    const connectId = async () => {
        props.client.connect()
        .then(accountAddress => {
            //console.log(accountAddress);
            props.updateAddress(accountAddress);
        }).catch((error) => {
            console.log(error);
      })
    }

    const createId = async () => {
        console.log("Creat concord Id");
    }

    const homeEvent = async () => {
        console.log("About");
    }

    const viewSteps = (view) => {
        switch(view) {
            case 0:
                setSubView(View[0])
                break;
            case 1:
                setSubView(View[1])
                break;
            case 2:
                setSubView(View[2])
                break;
        }
    }

    return (
        <div>
            <div className="d-flex justify-content-around match_window ">
            <div className="d-flex flex-column m-4 w-50">
                <div className='d-flex justify-content-between margin-right'>
                        <div>
                            <img className=" navbar-brand " onClick={homeEvent}  height={55} src="E-WillsLOGO.png" />
                            <img className=" navbar-brand " onClick={homeEvent}  height={35} src="E-WillsDTEXT.png" />
                        </div>
                </div>
                <div className=" w-100 margin-top-bottom text-center">
                    <h3 className="w-100 mt-5">Connect Your Concordium Wallet</h3>
                    <button type="button" onClick={connectId} className="btn btn-secondary btn-lg btn-block w-50 mt-5">Connect Wallet</button>
                    <div className="m-4"></div>
                    <button type="button" onClick={createId} className="btn w-50 bold-heavy"><u>Create Concordium Wallet</u></button>
                </div>
            </div>
           
                <div className="right-side-color w-50">

                </div>
            </div>
            
            
        </div>
       
    )

}

export default ConnectId;