
/* global BigInt */
import {
    concordiumSDK,
    AccountAddress,
    AccountTransactionType,
    CcdAmount,
    ModuleReference,
    deserializeReceiveReturnValue,
    serializeUpdateContractParameters,
    SchemaVersion,
    toBuffer,
} from '@concordium/web-sdk';

import contract from "./DeployedContract.json"

// Contract variables
// :5187
//5176
const contractIndex = 5514
const name = "ewillsabc"

class ContractServices {

    constructor(client) {
        this.client = client; 
        console.log(contract.rawModuleSchema);
        this.moduleReference = new ModuleReference('8998771bf7373def2c016cce72751955eb695b6ff88676b0ed753694787e43c6');
    }
    //b687b2a1587a1a160edb2f784ca950612be7be3021b0502c84e8bc1ff0a71675

    willCount(sender,will_id) {
        // set invoker account address
        let invoker = new AccountAddress(sender);
        const client = this.client;
        const wcPomise = new Promise(function(resolve,reject) {
            const param = serializeUpdateContractParameters (
                'ewillsabc',
                'will_count',
                {
                    token_id:will_id,
                    owner:invoker,
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract (
            {
                invoker: invoker, // set sender 
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewillsabc.will_count',
                parameter:param,
            },
            ).then((viewResult) => {
                // decode return values
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    "ewillsabc",
                    "will_count",
                    SchemaVersion.V2
                )
    
                // Int amount of wills from sender
                resolve(returnValue);
    
            }).catch((error) => {
                reject(error);
                //console.log(error)
                //alert(error)
            });
        });
        return wcPomise;
    }

    willExist(sender,will_id,testatorAddress) {
        // set invoker account address
        let invoker = new AccountAddress(sender);

        const param = serializeUpdateContractParameters (
            'ewillsabc',
            'will_exists',
            {
                will_id:will_id,
                owner:invoker,
            },
            toBuffer(contract.rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract (
        {
            invoker: invoker, // set sender 
            contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
            method:'ewillsabc.will_exists',
            parameter:param,
        },
        ).then((viewResult) => {
            // decode return values
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(contract.rawModuleSchema,'base64'),
                "ewillsabc",
                "will_exists",
                SchemaVersion.V2
            )
            console.log(returnValue);

        }).catch((error) => {
            console.log(error)
            alert(error)
        });
    }

    activeWill(will_id,invokerAddress) {
        let invoker = new AccountAddress(invokerAddress);
        const client = this.client;

        const wcPomise = new Promise(function(resolve,reject) {
            const param = serializeUpdateContractParameters(
                'ewillsabc',
                'active_will',
                {
                    will_id:will_id,
                    owner:invoker,
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewillsabc.active_will',
                parameter:param,
            }).then((viewResult) => {
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    "ewillsabc",
                    "active_will",
                    SchemaVersion.V2
                )
                resolve(returnValue);
            }).catch((error) => {
                reject(error);
            })
        });
        return wcPomise
    }

    isContract(invokerAddress) {
        let invoker = new AccountAddress(invokerAddress);
        //console.log(this.contract.rawModuleSchema);
        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract(
        {
            invoker: invoker,
            contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
            method:'ewillsabc.is_contract',
            
        }).then((viewResult) => {
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(contract.rawModuleSchema,'base64'),
                "ewillsabc",
                "is_contract",
                SchemaVersion.V2
            )
            console.log(returnValue);
        }).catch(alert)
    }

    getWills(sender,testatorAddress,will_id) {
        let invoker = new AccountAddress(sender);
        let testator = new AccountAddress(testatorAddress);
        const client = this.client;

        const wcPomise = new Promise(function(resolve,reject) {
            const param = serializeUpdateContractParameters(
                'ewillsabc',
                'get_will',
                {
                    will_id:will_id,
                    owner:testator,
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewillsabc.get_will',
                parameter:param,
            }).then((viewResult) => {
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    "ewillsabc",
                    "get_will",
                    SchemaVersion.V2
                )
                resolve(returnValue);
            }).catch((error) => {
                reject(error);
            })
        });
        return wcPomise
    }

    isWillNotarized(sender,will_id,testatorAddress) {
        let invokerAddress = new concordiumSDK.AccountAddress(sender);
        let ownerAddress = new concordiumSDK.AccountAddress(testatorAddress);
        const param = serializeUpdateContractParameters(
            'ewillsabc',
            'isNotarized',
            {
                will_id:will_id,
                owner:ownerAddress,
            },
            toBuffer(this.contract.rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract(
        {
            invoker: invokerAddress,
            contract:{index: BigInt(this.contractIndex), subindex: BigInt(0) },
            method:'ewillsabc.isNotarized',
            parameters:param,
        }).then((viewResult) => {
            let returnValue = concordiumSDK.deserializeReceiveReturnValue(
                concordiumSDK.toBuffer(viewResult.returnValue,"hex"),
                concordiumSDK.toBuffer(this.contract.rawModuleSchema,'base64'),
                "ewillsabc",
                "isNotarized",
                concordiumSDK.SchemaVersion.V2
            )
            console.log(returnValue);
        }).catch(alert)
    }

    notarize(notaryAddress,will_hash,will_id,testator,witness) {
        //console.log(notaryAddress);
        //console.log(will_hash);
        will_id = parseInt(will_id);
        //console.log(testator);
        //console.log(witness);
        const client = this.client;
        const nPromise = new Promise(function(resolve,reject) {
            // create `sendTransaction` object
            client.sendTransaction(
                notaryAddress,
                AccountTransactionType.Update,
                {
                    amount: new CcdAmount(0n),
                    contractAddress:{index: BigInt(contractIndex), subindex: BigInt(0) },
                    receiveName:"ewillsabc.notarize",
                    maxContractExecutionEnergy: 3000n

                },
                // Pass input parameters 
                {
                    will_hash,
                    will_id,
                    testator,
                    witness,
                },
                contract.rawModuleSchema
            ).then(txHash => {
                resolve(txHash);
            }).catch((error) => {
                reject(error)
            })
        })
        return nPromise
    }

    mint(testatorAddress,will_file,will_hash,notary) {
        const client = this.client;
        const mPomise = new Promise(function(resolve,reject) {
            // create `sendTransaction` object
            client.sendTransaction(
                testatorAddress,
                AccountTransactionType.Update,
                {
                    amount: new CcdAmount(0n),
                    contractAddress:{index: BigInt(contractIndex), subindex: BigInt(0) },
                    receiveName:"ewillsabc.mint",
                    maxContractExecutionEnergy: 3000n
                },
                // Pass input parameters 
                {
                    will_hash,
                    will_file,
                    notary,
                },
                contract.rawModuleSchema
            ).then(txHash => {
                resolve(txHash);
            }).catch((error) => {
                reject(error);
            })
        })
        return mPomise
    }

}

export default ContractServices