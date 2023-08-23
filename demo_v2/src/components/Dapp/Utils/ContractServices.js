
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
const contractIndex = 5952
const contractName = "ewills931"

class ContractServices {

    constructor(client) {
        this.client = client;
        this.moduleReference = new ModuleReference('b3df9734e0a3e1f99390020e5c67b8c3edf5bd4d9bc46eb86515b1529a0d0698');
    }

    willCount(sender,will_id) {
        // set invoker account address
        let invoker = new AccountAddress(sender);

        const client = this.client;
        const wcPomise = new Promise(function(resolve,reject) {

            const param = serializeUpdateContractParameters (
                contractName,
                'will_count',
                {
                    token_id:"00000000",
                    owner: {
                        Account: [sender],
                    },
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract (
            {
                invoker: invoker, // set sender 
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:contractName+'.will_count',
                parameter:param,
            },
            ).then((viewResult) => {
                // decode return values
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    contractName,
                    "will_count",
                    SchemaVersion.V2
                )
    
                // Int amount of wills from sender
                resolve(returnValue);
    
            }).catch((error) => {
                reject(error);
                console.log(error)
                //alert(error)
            });
        });
        return wcPomise;
    }

    willExist(sender,will_id,testatorAddress) {
        // set invoker account address
        let invoker = new AccountAddress(sender);

        const param = serializeUpdateContractParameters (
            contractName,
            'will_exists',
            {
                token_id:will_id,
                owner:{
                    Account: [sender],
                },
            },
            toBuffer(contract.rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract (
        {
            invoker: invoker, // set sender 
            contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
            method:contractName+'.will_exists',
            parameter:param,
        },
        ).then((viewResult) => {
            // decode return values
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(contract.rawModuleSchema,'base64'),
                contractName,
                "will_exists",
                SchemaVersion.V2
            )
            //console.log(returnValue);

        }).catch((error) => {
            console.log(error)
            alert(error)
        });
    }

    activeWill(will_id,sender) {
        let invoker = new AccountAddress(sender);
        const client = this.client;

        const wcPomise = new Promise(function(resolve,reject) {
            const param = serializeUpdateContractParameters(
                contractName,
                'active_will',
                {
                    token_id:will_id,
                    owner:{
                        Account: [sender],
                    },
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:contractName+'.active_will',
                parameter:param,
            }).then((viewResult) => {
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    contractName,
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
            method:contractName+'.is_contract',
            
        }).then((viewResult) => {
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(contract.rawModuleSchema,'base64'),
                contractName,
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
                contractName,
                'get_will',
                {
                    token_id:"00000000",
                    owner:{
                        Account: [testator],
                    },
                },
                toBuffer(contract.rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:contractName+'.get_will',
                parameter:param,
            }).then((viewResult) => {
                
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(contract.rawModuleSchema,'base64'),
                    contractName,
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

    getWillMeta(tokens) {
        const params = serializeUpdateContractParameters(
            
        )
    }

    isWillNotarized(sender,will_id,testatorAddress) {
        let invokerAddress = new concordiumSDK.AccountAddress(sender);
        let ownerAddress = new concordiumSDK.AccountAddress(testatorAddress);
        const param = serializeUpdateContractParameters(
            contractName,
            'isNotarized',
            {
                token_id:will_id,
                owner:{
                    Account: [ownerAddress],
                },
            },
            toBuffer(this.contract.rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract(
        {
            invoker: invokerAddress,
            contract:{index: BigInt(this.contractIndex), subindex: BigInt(0) },
            method:contractName+'.isNotarized',
            parameters:param,
        }).then((viewResult) => {
            let returnValue = concordiumSDK.deserializeReceiveReturnValue(
                concordiumSDK.toBuffer(viewResult.returnValue,"hex"),
                concordiumSDK.toBuffer(this.contract.rawModuleSchema,'base64'),
                contractName,
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
                    receiveName:contractName+".notarize",
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
                    receiveName:contractName+".mint",
                    maxContractExecutionEnergy: 3000n
                },
                // Pass input parameters 
                {
                    will_hash,
                    will_file,
                    notary: {
                        Account: [notary],
                    },
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