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
      //client:null,
      //account:null,
      //contractService:null,
      //willCount:null,
      //hasWills:null,
      //showNewWill:false,
      //showNotaryView:false,

      //prevWills:[],
      //activeWill:null,
    }

    //this.connect = this.connect.bind(this);
    //this.toggleShowView = this.toggleShowView.bind(this);
    //this.toggleNotaryView = this.toggleNotaryView.bind(this);
  }

  //componentWillMount() {
    //detectConcordiumProvider()
    //.then(c =>  {
    //  this.setState({client: c});
    //  this.setState({contractService: new ContractServices(c) });
    //})
    //.catch((error) => {
    //  console.log(error);
    //})
  //}

  //async connect() {
  //  this.state.client.connect()
  //  .then(accountAddress => {
  //    this.setState({account:accountAddress});
      // get users will count
  //    this.getWillCount(accountAddress);
  //  }).catch((error) => {
  //    console.log(error);
  //  })
  //}


  /*********** View Toggles ***********/

  //toggleShowView() {
  //  if (this.state.showNewWill) {
  //    this.setState({showNewWill:false});
  //  }else {
  //    this.setState({showNewWill:true});
  //  }
  //}

  //toggleNotaryView() {
  //  if (this.state.showNotaryView) {
  //    this.setState({showNotaryView:false});
  //  }else{
  //    this.setState({showNotaryView:true});
  //  }
  //}

 /*
  async getActiveWill(accountAddress) {
    this.state.contractService.activeWill(0,accountAddress)
    .then(willObject => {
      if (willObject.hasOwnProperty("Some")) {
        this.setState({activeWill:willObject.Some[0]})
      }
    })
    .catch((error) => {
      console.log(error);
    })
    
  }
 
  // get amount of wills testator has from contract
  async getWillCount(accountAddress) {
    //const willCountPromise = new Promise((resolve,reject) => {}
    this.state.contractService.willCount(accountAddress,0)
    .then(willCount => {
      this.setState({willCount:willCount});
      // get wills 
      if (willCount > 0){
        this.getWills(accountAddress,willCount)
        // get active will
        this.getActiveWill(accountAddress)
      }
    })
    .catch((error) => {
      console.log(error);
    })
  }
*/


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

  /*
  render() {
    return (
        <div    style={{
                backgroundColor: '#212529',
              }}>
          <Navi
            account = {this.state.account}
            showNewWill = {this.state.showNewWill}
            connect = {this.connect}
            toggleShowView = {this.toggleShowView}
          />
          { this.state.showNotaryView ? 
            <Notary
              account = {this.state.account}
              contractService = {this.state.contractService}
            />
          :
            <div>
            { this.state.showNewWill ?
                <span className="d-flex justify-content-center">
                  <CreateWill
                        account = {this.state.account}
                        contractServices = {this.state.contractService}
                    />
                </span>
                :
                <Main
                  account = {this.state.account}
                  willCount = {this.state.willCount}
                  wills = {this.state.prevWills}
                  contractServices = {this.state.contractService}
                  showNotaryView = {this.state.showNotaryView}
                  activeWill = {this.state.activeWill}
                />
              }
            </div>
          }


        
          <div className=' d-flex'>
            <button type="button" className="btn btn-primary btn-sm bottom-button" onClick={this.toggleNotaryView}>{this.state.showNotaryView ? "Testator" : "Notary"}</button>
          </div>
        </div>
    )
  }
  */
}


export default App;
