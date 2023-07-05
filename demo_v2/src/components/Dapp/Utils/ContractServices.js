
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

// Contract variables
// :5187
//5176
const contractIndex = 5187
const rawModuleSchema = "//8DAQAAAAkAAABld2lsbHMxNTYACAAAAAsAAABhY3RpdmVfd2lsbAIUAAIAAAAHAAAAd2lsbF9pZAIFAAAAb3duZXILFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAAFAAFAAAACQAAAHdpbGxfZmlsZRYCCQAAAHdpbGxfaGFzaB4gAAAABgAAAG5vdGFyeQsMAAAAaXNfbm90YXJpemVkAQYAAABlX3NlYWwUAAIAAAAJAAAAdGltZXN0YW1wFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAADQcAAAB3aXRuZXNzFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAACwgAAABnZXRfd2lsbAIUAAIAAAAHAAAAd2lsbF9pZAIFAAAAb3duZXILFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAAFAAFAAAACQAAAHdpbGxfZmlsZRYCCQAAAHdpbGxfaGFzaB4gAAAABgAAAG5vdGFyeQsMAAAAaXNfbm90YXJpemVkAQYAAABlX3NlYWwUAAIAAAAJAAAAdGltZXN0YW1wFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAADQcAAAB3aXRuZXNzFQIAAAAEAAAATm9uZQIEAAAAU29tZQEBAAAACwsAAABpc19jb250cmFjdAEBDAAAAGlzX25vdGFyaXplZAYUAAIAAAAHAAAAd2lsbF9pZAIFAAAAb3duZXILARUJAAAAEAAAAFBhcnNlUGFyYW1zRXJyb3ICBwAAAExvZ0Z1bGwCDAAAAExvZ01hbGZvcm1lZAILAAAAT25seUFjY291bnQCFAAAAE5vdGFyeUNhbnRCZVRlc3RhdG9yAg0AAABJbmNvcnJlY3RIYXNoAgYAAABOb1dpbGwCFAAAAFdpbGxBbHJlYWR5Tm90YXJpemVkAg8AAABJbmNvcnJlY3ROb3RhcnkCBAAAAG1pbnQEFAADAAAACQAAAHdpbGxfZmlsZRYCCQAAAHdpbGxfaGFzaB4gAAAABgAAAG5vdGFyeQsVCQAAABAAAABQYXJzZVBhcmFtc0Vycm9yAgcAAABMb2dGdWxsAgwAAABMb2dNYWxmb3JtZWQCCwAAAE9ubHlBY2NvdW50AhQAAABOb3RhcnlDYW50QmVUZXN0YXRvcgINAAAASW5jb3JyZWN0SGFzaAIGAAAATm9XaWxsAhQAAABXaWxsQWxyZWFkeU5vdGFyaXplZAIPAAAASW5jb3JyZWN0Tm90YXJ5AggAAABub3Rhcml6ZQQUAAQAAAAJAAAAd2lsbF9oYXNoHiAAAAAHAAAAd2lsbF9pZAIIAAAAdGVzdGF0b3ILBwAAAHdpdG5lc3MLFQkAAAAQAAAAUGFyc2VQYXJhbXNFcnJvcgIHAAAATG9nRnVsbAIMAAAATG9nTWFsZm9ybWVkAgsAAABPbmx5QWNjb3VudAIUAAAATm90YXJ5Q2FudEJlVGVzdGF0b3ICDQAAAEluY29ycmVjdEhhc2gCBgAAAE5vV2lsbAIUAAAAV2lsbEFscmVhZHlOb3Rhcml6ZWQCDwAAAEluY29ycmVjdE5vdGFyeQIKAAAAd2lsbF9jb3VudAIUAAIAAAAHAAAAd2lsbF9pZAIFAAAAb3duZXILAgsAAAB3aWxsX2V4aXN0cwIUAAIAAAAHAAAAd2lsbF9pZAIFAAAAb3duZXILAQA="
const name = "ewills156"

class ContractServices {

    constructor(client) {
        this.client = client; 
        this.moduleReference = new ModuleReference('8998771bf7373def2c016cce72751955eb695b6ff88676b0ed753694787e43c6');
    }
    //b687b2a1587a1a160edb2f784ca950612be7be3021b0502c84e8bc1ff0a71675

    willCount(sender,will_id) {
        // set invoker account address
        let invoker = new AccountAddress(sender);
        const client = this.client;
        const wcPomise = new Promise(function(resolve,reject) {
            const param = serializeUpdateContractParameters (
                'ewills156',
                'will_count',
                {
                    will_id:will_id,
                    owner:invoker,
                },
                toBuffer(rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract (
            {
                invoker: invoker, // set sender 
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewills156.will_count',
                parameter:param,
            },
            ).then((viewResult) => {
                // decode return values
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(rawModuleSchema,'base64'),
                    "ewills156",
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
            'ewills156',
            'will_exists',
            {
                will_id:will_id,
                owner:invoker,
            },
            toBuffer(rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract (
        {
            invoker: invoker, // set sender 
            contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
            method:'ewills156.will_exists',
            parameter:param,
        },
        ).then((viewResult) => {
            // decode return values
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(rawModuleSchema,'base64'),
                "ewills156",
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
                'ewills156',
                'active_will',
                {
                    will_id:will_id,
                    owner:invoker,
                },
                toBuffer(rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewills156.active_will',
                parameter:param,
            }).then((viewResult) => {
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(rawModuleSchema,'base64'),
                    "ewills156",
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
        //console.log(this.rawModuleSchema);
        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract(
        {
            invoker: invoker,
            contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
            method:'ewills156.is_contract',
            
        }).then((viewResult) => {
            let returnValue = deserializeReceiveReturnValue(
                toBuffer(viewResult.returnValue,"hex"),
                toBuffer(rawModuleSchema,'base64'),
                "ewills156",
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
                'ewills156',
                'get_will',
                {
                    will_id:will_id,
                    owner:testator,
                },
                toBuffer(rawModuleSchema, 'base64')
            );
            
            // create RPC invoke request
            client.getJsonRpcClient().invokeContract(
            {
                invoker: invoker,
                contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
                method:'ewills156.get_will',
                parameter:param,
            }).then((viewResult) => {
                let returnValue = deserializeReceiveReturnValue(
                    toBuffer(viewResult.returnValue,"hex"),
                    toBuffer(rawModuleSchema,'base64'),
                    "ewills156",
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
            'ewills156',
            'isNotarized',
            {
                will_id:will_id,
                owner:ownerAddress,
            },
            toBuffer(this.rawModuleSchema, 'base64')
        );

        // create RPC invoke request
        this.client.getJsonRpcClient().invokeContract(
        {
            invoker: invokerAddress,
            contract:{index: BigInt(this.contractIndex), subindex: BigInt(0) },
            method:'ewills156.isNotarized',
            parameters:param,
        }).then((viewResult) => {
            let returnValue = concordiumSDK.deserializeReceiveReturnValue(
                concordiumSDK.toBuffer(viewResult.returnValue,"hex"),
                concordiumSDK.toBuffer(this.rawModuleSchema,'base64'),
                "ewills156",
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
                    receiveName:"ewills156.notarize",
                    maxContractExecutionEnergy: 3000n

                },
                // Pass input parameters 
                {
                    will_hash,
                    will_id,
                    testator,
                    witness,
                },
                rawModuleSchema
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
                    receiveName:"ewills156.mint",
                    maxContractExecutionEnergy: 3000n
                },
                // Pass input parameters 
                {
                    will_hash,
                    will_file,
                    notary,
                },
                rawModuleSchema
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