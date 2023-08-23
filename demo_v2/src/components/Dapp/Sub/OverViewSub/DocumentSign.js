import React, { useState, useEffect } from "react";

import "./sub.css"


function DocumentSign(props) {

    const [files,setFiles] = useState([]);

    useEffect(() => {
        console.log(props);
    },[])

    return (
        <div className="p-4">

        </div>
    )
}
export default DocumentSign;