import React, { useState, useEffect } from "react";

import './ConnectId.css';

import Legal from "./OverViewSub/Legal";
import Documents from "./OverViewSub/Documents";
import Compliance from "./OverViewSub/Compliance";
import Notary from "./OverViewSub/Notary";
import DocumentSign from "./OverViewSub/DocumentSign";


function OverView(props) {

    const [subState,setSubState] = useState(null);

    useEffect(() => {
        setView(props.subState)
    },[props.subState,props.loading,props.willCount,props.wills])

    const setView = (state) => {

        switch(parseInt(state)) {
            case 0:
                setSubState(<Documents
                    loading = {props.loading}
                    willCount = {props.willCount}
                    wills = {props.wills}
                    setView = {setView}
                />);
                return
            case 1:
                setSubState(<Compliance/>);
                return
            case 2:
                setSubState(<Notary/>);
                return
            case 3:
                setSubState(<Legal/>);
                return
            case 4:
                setSubState(<DocumentSign/>);
                return
        }
    }

    return (
        <div className="container-fluid right-side-color">
                {subState}
        </div>
    )
}
export default OverView;