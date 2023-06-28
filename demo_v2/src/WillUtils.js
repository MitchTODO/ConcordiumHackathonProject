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