/*
  This script starts polling for any event produced by the precompiled
  LivingAssets contract, at address = 0x0000000000000000000000000000000000000402,
  starting from the latest block, and logs to console.
  Execute via:
    $ npm ci && node get_create_collection_events.js
*/

const { Web3 } = require('web3');
const web3 = new Web3(new Web3.providers.HttpProvider("https://arrakis.gorengine.com/own"));


const contractAddress = "0x0000000000000000000000000000000000000402";
const contractABI = [
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "collectionAddress",
        "type": "address"
      }
    ],
    "name": "CreateCollection",
    "type": "event"
  },
  {
    "inputs": [
      {
        "internalType": "string",
        "name": "baseURI",
        "type": "string"
      }
    ],
    "name": "createCollection",
    "outputs": [
      {
        "internalType": "address",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  }
];

const contract = new web3.eth.Contract(contractABI, contractAddress);

let lastBlockConsumed = 0;  // Initialize with the first block you want to start from

async function pollForEvents() {
    try {
        const latestBlock = await web3.eth.getBlockNumber();

        if (latestBlock > lastBlockConsumed) {
            console.log(`Consuming blocks from ${lastBlockConsumed + BigInt(1)} to ${latestBlock}`);

            const events = await contract.getPastEvents('allEvents', {
                fromBlock: lastBlockConsumed + BigInt(1),
                toBlock: 'latest'
            });

            for (let event of events) {
                console.log("New Event: ", event);
            }

            // Update the last block consumed
            lastBlockConsumed = latestBlock;
        } else {
            console.log("No new blocks since last check.");
        }

    } catch (error) {
        console.error(`Error fetching events: ${error}`);
    }

    // Poll every 10 seconds
    setTimeout(pollForEvents, 10000);
}

// Initialize with the latest block number
web3.eth.getBlockNumber().then(blockNumber => {
    lastBlockConsumed = blockNumber-BigInt(1);
    pollForEvents();  // Start the polling
});


