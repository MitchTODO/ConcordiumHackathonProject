import React, { useState, useEffect } from "react";
import HowItWorks from "./Sub/HowItWorks";
import RoadMap from "./Sub/RoadMap";
import About from "./Sub/About";
import MainPage from "./Sub/MainPage";
import Footer from "./Sub/Footer";
import "./Home.css"

function Home(props) {

  useEffect(() => {
  },[
    
  ])

  return (
    <div className="scrollable">
        <MainPage/>
        <About/>
        <HowItWorks/>
        <RoadMap/>
        <Footer/>
    </div>
  )

}
export default Home;
