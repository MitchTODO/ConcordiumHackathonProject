import React, { useState, useEffect } from "react";


function SideBar(props) {

    const [selectedView,setSelectedView] = useState(null);

    useEffect(() => {

    },[])

    const sideViews = (view) => {

        if (selectedView != null) {
            selectedView.classList.remove("selected");
        }
        
        const element = document.getElementById(view.target.id);

        setSelectedView(element)
        element.classList.add("selected");

        props.subState(view.target.id);
    }


    return (
        <div className="zero-padding side-bar ">
                <div className="d-flex flex-column flex-shrink-0 p-3 bg-light side-bar-height" >
                <a href="/" className="d-flex align-items-center mb-3 mb-md-0 me-md-auto link-dark text-decoration-none">
                    <div>
                            <img className=" navbar-brand "  height={45} src="E-WillsLOGO.png" />
                    </div>
                </a>
                <hr/>
                <ul className="nav nav-pills flex-column mb-auto">
                    <li>
                        <button className = "side-bar-button" onClick={sideViews} id = "0" >
                            Documents
                        </button>
                    </li>
                    <li>
                        <button className = "side-bar-button" onClick={sideViews} id = "1">
                            Compliance
                        </button>
                    </li>
                    <li>
                        <button className = "side-bar-button" onClick={sideViews} id = "2" >
                            Notarys
                        </button>
                    </li>
                    <li>
                        <button className = "side-bar-button" onClick={sideViews} id = "3" >
                            Legal
                        </button>
                    </li>
                </ul>
                <hr/>
                <div className="dropdown">
                    <a href="#" className="d-flex align-items-center link-dark text-decoration-none dropdown-toggle" id="dropdownUser2" data-bs-toggle="dropdown" aria-expanded="true">
                        <img src="https://github.com/mdo.png" alt="" width="32" height="32" className="rounded-circle me-2"/>
                        <strong>Name </strong>
                    </a>
                    <ul className="dropdown-menu text-small shadow" aria-labelledby="dropdownUser2">
                        <li><a className="dropdown-item" href="#">New project...</a></li>
                        <li><a className="dropdown-item" href="#">Settings</a></li>
                        <li><a className="dropdown-item" href="#">Profile</a></li>
                        <li><hr className="dropdown-divider"/></li>
                        <li><a className="dropdown-item" href="#">Disconnect</a></li>
                    </ul>
                </div>
                </div>
        </div>

    )

}
export default SideBar;