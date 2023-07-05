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

    const aboutId = async () => {
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
            <h3 className="text-center mt-5">Connect Your Concordium Digital Identity</h3>
            <div className="d-flex justify-content-center mt-5">
                <div className="d-flex flex-column m-4">
                    <button type="button" onClick={connectId} className="btn btn-secondary btn-lg btn-block">Connect Your Identity</button>
                    <div className="m-4"></div>
                    <button type="button" onClick={createId} className="btn btn-outline-secondary" disabled>Create Your Identity</button>
                </div>
            </div>
            <div  className="d-flex justify-content-center">
                <button type="button" onClick ={aboutId} className="btn m-2" disabled>What is Concordium Identity?</button>
            </div>
            
        </div>
       
    )

}

export default ConnectId;