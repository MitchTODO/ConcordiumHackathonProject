import {
    toBuffer,
} from '@concordium/web-sdk';

// SHA256 checksum 
// Parameters: `Buffer`
// Return: string (hash uf buffer)
export async function sha256(msgBuffer) {
    // hash the message
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);

    // convert ArrayBuffer to Array
    const hashArray = Array.from(new Uint8Array(hashBuffer));

    // convert bytes to hex string                  
    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    return hashHex;
}
/*
// Verifys Eseal of `Will`
function verifiyEseal(willFile) {
    will_text = willFile
    let buffer = toBuffer(willFile)
    // Promise for SHA256 hash
    new Promise((resolve, reject) => {
        resolve(sha256(buffer)); // get hash of will file buffer
    }).then((value) => {
        console.log(value);
    })
}
*/

// Gets transaction status until finalization
// Parameters: string (transaction hash)
export async function getTxStatus(client,txHash) {

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
                        
                        if (result.outcomes[outcomeKey].result.outcome == "success") {
                            resolve(true)
                        }else{
                            resolve(false)
                        }
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