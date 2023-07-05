import React, { useState, useEffect } from "react";
import CreateWill from "../CreateWill";

import WillView from "./WillView";


function PreviousWills(props) {

    const [userView, setUserView] = useState(null);
    const [hasLoadWills,setHasLoadWills] = useState(false);
    const [willViews,setWillViews] = useState([]);

    useEffect(() => {
        // use enum for user state
        // set user view
        loadWills()
    },[props.willCount,props.wills])

    function loadWills() { 
        let views = []
        for(let w = 0; w <= props.willCount - 1; w++) {
            let willObject = null 
            if (props.wills != null) {
                willObject = props.wills[w];
            }
            let willView = <WillView
                                key = {w}
                                will = {willObject}
                            />
            views.push(willView)
        }
        setWillViews(views);
    }

    return (
        <div>
            <div className="container bg-secondary rounded">
                <div className="d-flex justify-content-between pt-1">
                        <h5 className="card-title text-center text-light m4">
                           All Wills
                        </h5>
                        <button type="button" className="btn btn-primary" onClick={props.createWillAction} >Create New Will</button>
                </div>
                <div className="row flex-nowrap overflow-auto">
                    {willViews}
                </div>
            </div>
        </div>

    )

}
export default PreviousWills;