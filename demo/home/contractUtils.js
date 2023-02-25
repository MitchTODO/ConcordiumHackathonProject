
// State object
const TestatorState = {
	Loading: "Loading",
	hasWill: true,
	noWill: false,
	Default: null,
}

const NotaryState = {
    Loading: "Loading",
    hasWill:true,
    noWill:false,
}

// view state
let activeViewId = 0; // set to Testator state
let testatorState = TestatorState.Default; // states within testator

// concordiumSDK client 
let client = null;
// Connected address 
let currentAccountAddress = null;


// will creation
let hasWill = null
let original_hash = null
let will_hash = ""
let will_file = "" 
let will_text = ""

// will notarization
let willId = null;
let willOnSearch = null;
let timeStamp = null;

// Contract variables
const contractIndex = 3427
const rawModuleSchema = "//8DAQAAAAkAAABld2lsbHMxNDYACAAAAAQAAABidXJuAxUKAAAAEAAAAFBhcnNlUGFyYW1zRXJyb3ICBwAAAExvZ0Z1bGwCDAAAAExvZ01hbGZvcm1lZAILAAAAT25seUFjY291bnQCFAAAAE5vdGFyeUNhbnRCZVRlc3RhdG9yAhUAAABXaWxsQWxyZWFkeU93bmVkRXJyb3ICDAAAAE5vV2lsbFRvQnVybgIGAAAATm9XaWxsAhQAAABXaWxsQWxyZWFkeU5vdGFyaXplZAIPAAAASW5jb3JyZWN0Tm90YXJ5AgcAAABnZXRXaWxsAgsVAgAAAAQAAABOb25lAgQAAABTb21lAQEAAAAUAAUAAAAJAAAAd2lsbF9maWxlFgIJAAAAd2lsbF9oYXNoHiAAAAAGAAAAbm90YXJ5CwwAAABpc19ub3Rhcml6ZWQBBgAAAGVfc2VhbBQAAgAAAAkAAAB0aW1lc3RhbXAVAgAAAAQAAABOb25lAgQAAABTb21lAQEAAAANBwAAAHdpdG5lc3MVAgAAAAQAAABOb25lAgQAAABTb21lAQEAAAALEQAAAGdldFdpbGxGcm9tU2VuZGVyARUCAAAABAAAAE5vbmUCBAAAAFNvbWUBAQAAABQABQAAAAkAAAB3aWxsX2ZpbGUWAgkAAAB3aWxsX2hhc2geIAAAAAYAAABub3RhcnkLDAAAAGlzX25vdGFyaXplZAEGAAAAZV9zZWFsFAACAAAACQAAAHRpbWVzdGFtcBUCAAAABAAAAE5vbmUCBAAAAFNvbWUBAQAAAA0HAAAAd2l0bmVzcxUCAAAABAAAAE5vbmUCBAAAAFNvbWUBAQAAAAsLAAAAaXNOb3Rhcml6ZWQGCwEVCgAAABAAAABQYXJzZVBhcmFtc0Vycm9yAgcAAABMb2dGdWxsAgwAAABMb2dNYWxmb3JtZWQCCwAAAE9ubHlBY2NvdW50AhQAAABOb3RhcnlDYW50QmVUZXN0YXRvcgIVAAAAV2lsbEFscmVhZHlPd25lZEVycm9yAgwAAABOb1dpbGxUb0J1cm4CBgAAAE5vV2lsbAIUAAAAV2lsbEFscmVhZHlOb3Rhcml6ZWQCDwAAAEluY29ycmVjdE5vdGFyeQIEAAAAbWludAQUAAMAAAAJAAAAd2lsbF9maWxlFgIJAAAAd2lsbF9oYXNoHiAAAAAGAAAAbm90YXJ5CxUKAAAAEAAAAFBhcnNlUGFyYW1zRXJyb3ICBwAAAExvZ0Z1bGwCDAAAAExvZ01hbGZvcm1lZAILAAAAT25seUFjY291bnQCFAAAAE5vdGFyeUNhbnRCZVRlc3RhdG9yAhUAAABXaWxsQWxyZWFkeU93bmVkRXJyb3ICDAAAAE5vV2lsbFRvQnVybgIGAAAATm9XaWxsAhQAAABXaWxsQWxyZWFkeU5vdGFyaXplZAIPAAAASW5jb3JyZWN0Tm90YXJ5AggAAABub3Rhcml6ZQQUAAMAAAAJAAAAd2lsbF9oYXNoHiAAAAAHAAAAd2lsbF9pZAsHAAAAd2l0bmVzcwsVCgAAABAAAABQYXJzZVBhcmFtc0Vycm9yAgcAAABMb2dGdWxsAgwAAABMb2dNYWxmb3JtZWQCCwAAAE9ubHlBY2NvdW50AhQAAABOb3RhcnlDYW50QmVUZXN0YXRvcgIVAAAAV2lsbEFscmVhZHlPd25lZEVycm9yAgwAAABOb1dpbGxUb0J1cm4CBgAAAE5vV2lsbAIUAAAAV2lsbEFscmVhZHlOb3Rhcml6ZWQCDwAAAEluY29ycmVjdE5vdGFyeQIKAAAAd2lsbEV4aXN0cwILARQAAAB3aWxsRXhpc3RzRnJvbVNlbmRlcgEBAA"
const moduleReference = new concordiumSDK.ModuleReference('30728b0b2bc2bee6bdc43f34d618a060d907b8b8b76fb473b6dffe366364403a');

// initalize view & concord client
function initStates() {
    toggleView(activeViewId)

    concordiumHelpers.detectConcordiumProvider()
    .then(c => client = c)
    .catch((error) => {
        console.log(error);
    });

}

// Init setup when page is loaded
addEventListener("DOMContentLoaded",(_) => initStates());

// Connect browser extension
function connect() {
    // connect client 
    client.connect()
    .then(accountAddress => {
        // set connected address
        currentAccountAddress = accountAddress
        showNotaryWill(true)
        // get will status for connected address
        willExist()
        // set & display connected address
        document.getElementById("tx_cont").style.display = "block";
        document.getElementById("accountAddress").innerHTML = accountAddress;
    })
    .catch((error) => {
        console.log(error);
    })
}

/******************* View Methods *******************/

// Toggle between testator & notary view
function toggleView(viewId) {
    if (viewId == 0) {
        document.getElementById("testator-view").style.display = "block";
        document.getElementById("notary-view").style.display = "none";
        
    }else{
        document.getElementById("notary-view").style.display = "block";
        document.getElementById("testator-view").style.display = "none";
    }
    document.getElementById(activeViewId).classList.remove("active");
    document.getElementById(viewId).classList.add("active");

    activeViewId = viewId
}

// Toggle subviews within testator view
function toggleTestator(viewState) {

    testatorState = viewState
    // Set all views to hidden
    document.getElementById("will-view").style.display = "none";
    document.getElementById("create-will").style.display = "none";
    document.getElementById("progress-view").style.display = "none";
    document.getElementById("default-view").style.display = "none";
    // Switch correct view to `block`
    switch (testatorState) {
        case true:
            document.getElementById("will-view").style.display = "block";
            break;

        case false:
            document.getElementById("create-will").style.display = "block";
            break;

        case "Loading":
            document.getElementById("progress-view").style.display = "block";
            break;

        case null:
            document.getElementById("default-view").style.display = "block";
            break;
    }
}

// Set & display notary will view
function showNotaryWill(toShow) {
    // if will object exist 
    if(toShow && willOnSearch != null){
        // if will object is not None
        if (willOnSearch.hasOwnProperty("Some")) {
            will_file = willOnSearch.Some[0].will_file
            // Get will file passing will object
            getWillFile(will_file,willOnSearch)
            // Display notary will view
            document.getElementById("lookup_error").style.display = "none";
            document.getElementById("notary_signing").style.display = "block";
            // Set notary views
            document.getElementById("will_hash_2").textContent = willOnSearch.Some[0].will_hash;
            document.getElementById("notary_status_2").textContent = willOnSearch.Some[0].is_notarized;
            document.getElementById("notary_address_2").textContent = willOnSearch.Some[0].notary;
            if (willOnSearch.Some[0].is_notarized) {
                document.getElementById("notary_status_2").textContent = "✅ Successfully Notarized"
            }else{
                document.getElementById("notary_status_2").textContent = "🔴 Will has not been notarized."
            }

            // Set will notary overview status
            // This is presented on-top of will view
            let element = document.getElementById("active_notary");
            if(currentAccountAddress == null) {
                element.textContent = "🔴 Connect Wallet to notarize will."
            }else if (willOnSearch.Some[0].notary != currentAccountAddress) {
                element.textContent = "🔴 You are not the acting notary for this will."
            }else {
                element.textContent = "✅ You are the acting notary for this will."
            }
            
            if (willOnSearch.Some[0].is_notarized){
                element.textContent = "✅ Will has already been notarized."
                document.getElementById("esealed").style.display = "none";
            }

        }else{
            // Show lookup error if results are empty
            document.getElementById("lookup_error").style.display = "block";
            document.getElementById("notary_signing").style.display = "none";
        }
        
    }else{
        // No will, hide views
        document.getElementById("notary_signing").style.display = "none";
    }
 
}

// Set & display testator will view
function showTestatorWill(willObject) {
    // If will object is not None
    if (willObject.hasOwnProperty("Some")) {
        will_file = willObject.Some[0].will_file
        // Set original will hash 
        // Check against when verifiying eseal
        original_hash = willObject.Some[0].will_hash;
        // Set will information on page
        document.getElementById("notary_address").textContent = willObject.Some[0].notary;
        document.getElementById("mint_hash").textContent = willObject.Some[0].will_hash;
        document.getElementById("will_url").textContent = willObject.Some[0].will_file;
        if (willObject.Some[0].is_notarized) {
            document.getElementById("notary_status").textContent = "✅ Successfully Notarized"
        }else{
            document.getElementById("notary_status").textContent = "🔴 Will has not been notarized."
        }
        
        // Get will file passing will object
        getWillFile(will_file,willObject)
    }else{
        alert("Failed to read Will information.");
    }
}



/******************* Contract Utils *******************/

// Checks if `Will` has been Esealed
// Returns: bool
function isEseal(eseal) {
    // if timestamp and witness keys exists in eseal object
    let hasTimeStamp = eseal.timestamp.hasOwnProperty("Some");
    let hasWitness = eseal.witness.hasOwnProperty("Some");
    if (hasWitness && hasTimeStamp) {
        return true
    }else{
        return false
    }
}

// Verifys Eseal of `Will`
function verifiyEseal(willFile,will) {
    will_text = willFile
    let buffer = concordiumSDK.toBuffer(willFile)
    // Promise for SHA256 hash
    new Promise((resolve, reject) => {
        resolve(sha256(buffer)); // get hash of will file buffer
    }).then((value) => {
        // if `0` set views for testator 
        if (activeViewId == 0) {
            // only show hash on will creation
            if(hasWill == false) {
                will_hash = value
                document.getElementById("willHash").innerText = value;
                return
            }
            // show full will view 
            document.getElementById("temp-will").style.display = "none";
            document.getElementById("testator-will").style.display = "block";
            // set content in will view
            document.getElementById("testator-will").innerText = will_text;
            document.getElementById("testator-current-hash").innerText = value;

            // get & set Eseal status 
            if(!isEseal(will.Some[0].e_seal)) {
                document.getElementById("eseal_status").textContent = "🔴 Will has not been E-Sealed"
            }else if(original_hash != value ) {
                document.getElementById("time_stamp").textContent = will.Some[0].e_seal.timestamp.Some[0];
                document.getElementById("witness_address").textContent = will.Some[0].e_seal.witness.Some[0];
                document.getElementById("eseal_status").innerText =  "⛔️ Invalid Eseal Verification! Will has been changed from it's original sealed state."
            }else{
                document.getElementById("time_stamp").textContent = will.Some[0].e_seal.timestamp.Some[0];
                document.getElementById("witness_address").textContent = will.Some[0].e_seal.witness.Some[0];
                document.getElementById("eseal_status").innerText =  "✅ Successfully Verfied"
            }
        }else{ 
            // set views for notary
            will_hash = value
            document.getElementById("notary-will").innerText = will_text;
            document.getElementById("notary-will").style.display = "block";
            document.getElementById("temp-n-will").style.display = "none"
            document.getElementById("current_will_hash_2").innerText = value;
            // Varifiy `will` Eseal
            // check `will` has been through esealing process
            if(!isEseal(will.Some[0].e_seal)) {
                document.getElementById("eseal_status_2").textContent = "🔴 Will has not been E-Sealed"
            }else if(willOnSearch.Some[0].will_hash != value ) { // check current will file hash with original hash from `mint`
                document.getElementById("time_stamp_2").textContent = will.Some[0].e_seal.timestamp.Some[0];
                document.getElementById("witness_address_2").textContent = will.Some[0].e_seal.witness.Some[0];
                document.getElementById("eseal_status_2").innerText =  "⛔️ Invalid Eseal Verification! Will has been changed from it's original sealed state."
            }else{ // all checks pass then eseal is verified
                document.getElementById("time_stamp_2").textContent = will.Some[0].e_seal.timestamp.Some[0];
                document.getElementById("witness_address_2").textContent = will.Some[0].e_seal.witness.Some[0];
                document.getElementById("eseal_status_2").innerText =  "✅ Successfully Verfied"
            }
        }
        
    })
}

// Match contract error code to string 
// Returns: string
function filterContractErrors(errorId) {
    switch (errorId) {
        case -1:
            return("Incorrect Parameters.");
        case -2:
            return("Contract Log is full.");
        case -3:
            return("Contract log is malformed.");
        case -4:
            return("Only Accounts can call contract methods.");
        case -5:
            return("Notary cant be the testator on the same will.");
        case -6:
            return("Sender Address already ownes a will.");
        case -7:
            return("Sender Address has not will to burn.");
        case -8:
            return("Sender Address has not will.");
        case -8:
            return("Will has already been notarized.");
        case -9:
            return("Incorrect Notary for will.");
        default:
          return("Unknown Error");
      }
}

// Gets transaction status until finalization
// Parameters: string (transaction hash)
async function getTxStatus(txHash) {
    // Create & display transaction hash and progress view
    let text = document.createTextNode(txHash);
    let progressView = document.createElement("div");
    progressView.classList.add("small-loader");
    progressView.id = txHash+"progress";
    text.id = txHash;
    
    let container = document.createElement("div");
    container.classList.add("bottom-border");
    container.append(text);
    // Append progress view to `tx_history`
    container.append(progressView);
    document.getElementById("tx_history").appendChild(container);
    // Start async interval request for status
    // request are 1 second intervals
    return await new Promise(resolve => {
        const interval = setInterval(() => {
                client.getJsonRpcClient().getTransactionStatus(txHash)
                .then((result) => {
                    // if tx is in finalized status
                    if(result.status == "finalized") {
                        // Get tx status data
                        let outcomeKey = Object.keys(result.outcomes)[0]
                        let progressView = document.getElementById(result.outcomes[outcomeKey].hash+"progress")
                        let parent = progressView.parentElement;
                        let statusView;
                        if (result.outcomes[outcomeKey].result.outcome == "success") {
                            statusView = document.createTextNode("✅");
                            if (activeViewId == 0) {
                                // Get new will state 
                                // Note: Changes view based on `will` state
                                willExist() 
                            }
                        }else{
                            // Show error
                            try {
                                let contractError = filterContractErrors(result.outcomes[outcomeKey].result.rejectReason.rejectReason)
                                statusView = document.createTextNode("🔴 \n "+ contractError);
                            }catch(err) {
                                statusView = document.createTextNode("🔴 \n: Unkown error");
                            }
                        }
                        // remove progress & show status
                        progressView.remove()
                        parent.appendChild(statusView)
                        // clear interval
                        clearInterval(interval);
                    }

                }).catch((error) => {
                    alert(error)
                    clearInterval(interval);
                });
            
          }, 1000);
    });
}

// Get will file from url
// Parameters: `Will` will object 
function getWillFile(willFile,will) {
    will_file = willFile
    fetch(willFile, {mode: 'cors'})
        .then((response) => response.body)
        .then((rb) => {
            const reader = rb.getReader();
            return new ReadableStream({
                start(controller) {
                    // handle pushed data chunk
                    function push() {
                    // "done" is a Boolean and value a "Uint8Array"
                    reader.read().then(({ done, value }) => {
                        // If there is no more data to read
                        if (done) {
                            controller.close();                            
                            return;
                        }
                        // Get the data and send it to the browser via the controller
                        controller.enqueue(value);
                        
                        push();
                    });
                    }
                    push();
                },
            });
        })
        .then((stream) =>
            // Respond with our stream
            new Response(stream, { headers: { "Content-Type": "text/html" } }).text()
        )
        .then((result) => {
            // verify will file 'result' is string 'will' will object
            verifiyEseal(result,will);
        })
        .catch((error) => {
            alert(error);
        });
}

// SHA256 checksum 
// Parameters: `Buffer`
// Return: string (hash uf buffer)
async function sha256(msgBuffer) {
    // hash the message
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);

    // convert ArrayBuffer to Array
    const hashArray = Array.from(new Uint8Array(hashBuffer));

    // convert bytes to hex string                  
    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    return hashHex;
}

/******************* Contract Methods *******************/

// Writes to `Burn` contract method
// Destruction of a existing `will`
function burn() {
    // create `sendTransaction` object
    client.sendTransaction(
        currentAccountAddress,
        client.sendTransaction(
            currentAccountAddress,
            concordiumSDK.AccountTransactionType.Update,
            // set transaction values with will contract variables
            {
                amount: new concordiumSDK.CcdAmount(0n),
                contractAddress:{index: BigInt(contractIndex), subindex: BigInt(0) },
                receiveName:"ewills146.burn", 
                maxContractExecutionEnergy: 3000n
            },
            // No parameter objects
            {

            },
            rawModuleSchema
        ).then(txHash => {
            // Get transaction status
            getTxStatus(txHash)
        }).catch(alert)
    )
}

// Writes to `Mint` contract method
// Creation of a new `will`
function mint() {
    // Check input parameters
    let notary = document.getElementById("notary").value;
    
    if(will_file == "" || will_hash == "" || notary == "") {
        alert("Missing parameters, please check your inputs.")
        return
    }

    // Create `sendTransaction` object
    client.sendTransaction(
        currentAccountAddress,
        concordiumSDK.AccountTransactionType.Update,
        // Set transaction values with will contract variables
        {
            amount: new concordiumSDK.CcdAmount(0n),
            contractAddress:{index: BigInt(contractIndex), subindex: BigInt(0) },
            receiveName:"ewills146.mint",
            maxContractExecutionEnergy: 3000n
        },
        // Pass input parameters
        // Note: Var names MUST match contract input names
        {
            will_hash,
            will_file,
            notary,
        },
        // Contract schema
        rawModuleSchema
    ).then(txHash => {
        // Get transaction status
        getTxStatus(txHash)
    }).catch(alert)

}

// Writes to `notarize` contract method
// Notarization of a existing `will`
function notarize() {

    // Get input parameters
    let will_id = document.getElementById("lookup").value; // Account Address of owner 
    let witness = document.getElementById("witness_input").value; // Account Address of witness 

    // create `sendTransaction` object
    client.sendTransaction(
        currentAccountAddress,
        concordiumSDK.AccountTransactionType.Update,
        {
            amount: new concordiumSDK.CcdAmount(0n),
            contractAddress:{index: BigInt(contractIndex), subindex: BigInt(0) },
            receiveName:"ewills146.notarize",
            maxContractExecutionEnergy: 3000n

        },
        // Pass input parameters 
        {
            will_hash,
            will_id,
            witness,
        },
        rawModuleSchema
    ).then(txHash => {
        getTxStatus(txHash)
    }).catch(alert)
}

// Invokes `willExistFromSender` contract method
function willExist() {
    // Toggle views to loading
    toggleTestator(TestatorState.Loading)
    // set invoker account address
    let invoker = new concordiumSDK.AccountAddress(currentAccountAddress);
    // create RPC invoke request
    client.getJsonRpcClient().invokeContract(
    {
        invoker: invoker, // set sender 
        contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
        method:'ewills146.willExistsFromSender',
    },
    
    ).then((viewResult) => {
        // decode return values
        let returnValue = concordiumSDK.deserializeReceiveReturnValue(
            concordiumSDK.toBuffer(viewResult.returnValue,"hex"),
            concordiumSDK.toBuffer(rawModuleSchema,'base64'),
            "ewills146",
            "willExistsFromSender",
            concordiumSDK.SchemaVersion.V2
        )
        // Get will if true
        if (returnValue) {
            getWill(invoker)
        }
        // set will status
        hasWill = returnValue
        // Toggle to correct view
        toggleTestator(returnValue)
    }).catch((error) => {
        alert(error)
        // Toggle to default view
        toggleTestator(TestatorState.Default)
    });
}

// Invokes `getWill` contract method
function getWill(sender) {
    // replace sender with lookup address from button press
    if(sender == null) { // check if sender is null
        // get address from lookup button
        let searchAddress = document.getElementById("lookup").value;
        try{
            // set sender to lookup AccountAddress
            sender = new concordiumSDK.AccountAddress(searchAddress);
        }catch(error){
            alert(error)
        }
    }
    // create RPC invoke request
    client.getJsonRpcClient().invokeContract(
    {
        invoker: sender,
        contract:{index: BigInt(contractIndex), subindex: BigInt(0) },
        method:'ewills146.getWillFromSender',
     
    }).then((viewResult) => {
        let returnValue = concordiumSDK.deserializeReceiveReturnValue(
            concordiumSDK.toBuffer(viewResult.returnValue,"hex"),
            concordiumSDK.toBuffer(rawModuleSchema,'base64'),
            "ewills146",
            "getWillFromSender",
            concordiumSDK.SchemaVersion.V2
        )
        // check view state 
        if(activeViewId == 1){
            // set will object 
            willOnSearch = returnValue
            // Update notary views
            showNotaryWill(true)
   
        }else{
            // Update testator views
            showTestatorWill(returnValue)
        }
    
    }).catch(alert)
}

