import React, {Component} from 'react';

import {
  BrowserRouter,
  Routes,
  Route,
  Link
} from "react-router-dom";

//import { detectConcordiumProvider } from '@concordium/browser-wallet-api-helpers';

//import ContractServices from './ContractServices';

import './App.css';
import Navi from './NavBar';

import Home from "./components/Home/Home";
import Dapp from './components/Dapp/Dapp';
import Notary from './components/Dapp/Notary.js'

class App extends Component {

  constructor(props) {
    super(props)
    
    this.state = {
    }

  }

  render() {
    return (
      <div style={{
        backgroundColor: '#fff',
      }}>
        <BrowserRouter>
          <Routes>
            <Route path="/" element ={
              <div>
                <Navi/>
                <Home/>
              </div>
            }/>
          </Routes>
          <Routes>
            <Route path="/app" element ={
              <Dapp/>
            }/>
          </Routes>
          <Routes>
            <Route path="/notary" element ={
              <Notary/>
            }/>
          </Routes>
        </BrowserRouter>
      </div>

    )
  }
}


export default App;
