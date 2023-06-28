import React, { Component } from 'react'

import logo from './assets/logo.png';

class Navi extends React.Component {

  constructor(props){
    super(props);

      this.handleAccount = this.handleAccount.bind(this)
      this.toggleCreationView = this.toggleCreationView.bind(this)
 
  }

  // loads account from navi button
  async handleAccount() {
    this.props.connect()
  }

  toggleCreationView() {
    this.props.toggleShowView()
  }

  render() {
    return (
      <div>
        <nav className="navbar navbar-dark bg-dark">
            <img className="navbar-brand m-2"  height={80} src={logo} />
            <div className=" p-2 bd-highlight">
              <ul className="navbar-nav d-flex flex-row">
                <li className="nav-item px-2">
                    {!this.props.account ? (
                        <button type="button" onClick ={this.handleAccount} className="btn  btn-success m-2">Connect Wallet</button>
                      ):(
                        <button type="button" className="btn btn-success m-2" onClick={this.handleAccount} >{this.props.account}</button>
                      )
                    }
                </li>
                <li className="nav-item px-2">
                  { this.props.account && 
                   <button type="button" className="btn btn-success m-2" onClick={this.toggleCreationView} >{this.props.showNewWill ? "Exit": "New Will"}</button>
                  }
                 
                </li>
              </ul>
              </div>
          </nav>
        </div>


    );
  }
}



export default Navi;