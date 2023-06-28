import React, { useState, useEffect } from "react";
import CreateWill from "./CreateWill";
import '../App.css';
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
        console.log(props.wills);
        for(let w = 0; w <= props.willCount - 1; w++){
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
                <h5 className="card-title text-center text-light m4">Your Minted Wills</h5>
                <div className="row flex-nowrap overflow-auto">
                    {willViews}
                </div>
            </div>
        </div>

    )

}
export default PreviousWills;