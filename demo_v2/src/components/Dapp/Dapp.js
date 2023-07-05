import React, {Component} from 'react';
import logo from "../assets/logoLight.png";
import './Dapp.css'
import ConnectId from "./Sub/ConnectId";
import WillOverView from './Sub/WillOverView';

import { detectConcordiumProvider } from '@concordium/browser-wallet-api-helpers';
import ContractServices from './Utils/ContractServices';

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
            <div style={{
                backgroundColor: '#212529',
                }}>

                
                <div className='d-flex justify-content-between margin-right'>
                    <img className=" p-2 navbar-brand m-2 "  height={55} src={logo} />
                    
                    <div className=" d-inline-block topAddress rounded-bottom bg-primary p-1 text-center">
                        <b>Identity Address</b>
                        <p className='p-1 text-truncate font-style' >{this.state.account}</p>
                        <div className='d-flex justify-content-center '>
                            <button type="button" className="btn btn-outline-primary btn-sm" >Copy</button>
                            
                            <button type="button" className="btn btn-outline-primary btn-sm" onClick={this.connectId} >Disconnect</button>
                        </div>
                    </div>
                </div>

                {this.viewSteps()}
            </div>
        )
    }

}

/*
function Dapp(props) {
    const [subView,setSubView] = useState();

    useEffect(() => {
        viewSteps(0)
    },[
    ])

    const View = {
        0: <ConnectId/>,
        1: <div>Create your will</div>,
        2: <div>Notary</div>
    }

    const viewSteps = (view) => {
        switch(view) {
            case 0:
                setSubView(View[0])
                break;
            case 1:
                setSubView(View[1])
                break;
            case 2:
                setSubView(View[2])
                break;
        }
    }
    return (
        <div style={{
            backgroundColor: '#212529',
          }}>
            <img className=" p-2 navbar-brand m-2 "  height={55} src={logo} />
            {subView}
        </div>
    )
}
*/

export default Dapp;