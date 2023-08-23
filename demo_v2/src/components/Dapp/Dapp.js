import React, {Component} from 'react';
import './Dapp.css'
import ConnectId from "./Sub/ConnectId";
import WillOverView from './Sub/WillOverView';

import { detectConcordiumProvider } from '@concordium/browser-wallet-api-helpers';
import ContractServices from './Utils/ContractServices';

// TODO change to enum conditional rendering
class Dapp extends Component {

    constructor(props){
        super(props)

        this.state = {
            contractService:null,
            client:null,
            account:null,
            viewId:"A",

        }
    
        this.viewSteps = this.viewSteps.bind(this);
        this.updateAddress = this.updateAddress.bind(this);
        this.connectId = this.connectId.bind(this);
    }

    componentWillMount() {
        detectConcordiumProvider()
        .then(c =>  {
            this.setState({client: c});
            this.setState({contractService: new ContractServices(c) });
        })
        .catch((error) => {
            console.log(error);
        })
    }

    updateAddress(address) {
        // switch views to 
        if (address != null) {
            this.setState({viewId: "B"});
        }
        this.setState({account:address});
    }

    connectId() {
        if (this.state.client != null) {
            this.state.client.connect()
            .then(accountAddress => {
                this.updateAddress(accountAddress);
            }).catch((error) => {
                console.log(error);
          })
        }
    }

    viewSteps() {
        switch(this.state.viewId) {
            case "A":
                return <ConnectId
                        client = {this.state.client}
                        contractService = {this.state.contractService}
                        updateAddress = {this.updateAddress}
                        />
            case "B":
                return <WillOverView
                        client = {this.state.client}
                        contractService = {this.state.contractService}
                        account = {this.state.account}
                        />
            case "C":
                return <div></div>
        }
    }

    render() {
        return(
            <div style={{backgroundColor: '#b7c5d2'}}>
                
                {this.viewSteps()}
            </div>
        )
    }

}

export default Dapp;