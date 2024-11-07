import { Contract, Account, Keypair, Server, networks, TransactionBuilder } from '@stellar/stellar-sdk';

const API_ENDPOINT = 'https://www.xe.com/api/protected/statistics/?from=USD&to=NGN';

// stellar network
const kp = Keypair.random();
const secret = kp.secret();
const pubKey = kp.publicKey();
console.log('Soroban Secret: ', secret);
console.log('Soroban Account: ', pubKey);


// send it to your backend
fetch(API_ENDPOINT, {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({ 
    "alice": alice,
    "bob": bob
  },
)
});
