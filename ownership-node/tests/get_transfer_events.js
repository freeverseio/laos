/*
  This script starts polling for transfer events produced by one of the ERC721 contracts
  associated to created collections, starting from the latest block, and logs to console.
  Execute via:
    $ npm ci && node get_transfer_events.js
*/

const { Web3 } = require('web3');
const web3 = new Web3(new Web3.providers.HttpProvider("https://arrakis.gorengine.com/own"));

// provide the ERC721 address associated to a created collection:
const contractAddress = "0xFfffffFFfFFFfffFFFFFffFf0000000000000027";
const contractABI = [
	{
		"anonymous": false,
		"inputs": [
			{
				"indexed": true,
				"internalType": "address",
				"name": "_from",
				"type": "address"
			},
			{
				"indexed": true,
				"internalType": "address",
				"name": "_to",
				"type": "address"
			},
			{
				"indexed": true,
				"internalType": "uint256",
				"name": "_tokenId",
				"type": "uint256"
			}
		],
		"name": "Transfer",
		"type": "event"
	},
	{
		"inputs": [
			{
				"internalType": "address",
				"name": "_from",
				"type": "address"
			},
			{
				"internalType": "address",
				"name": "_to",
				"type": "address"
			},
			{
				"internalType": "uint256",
				"name": "_tokenId",
				"type": "uint256"
			}
		],
		"name": "transferFrom",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint256",
				"name": "_tokenId",
				"type": "uint256"
			}
		],
		"name": "ownerOf",
		"outputs": [
			{
				"internalType": "address",
				"name": "",
				"type": "address"
			}
		],
		"stateMutability": "view",
		"type": "function"
	},
	{
		"inputs": [
			{
				"internalType": "uint256",
				"name": "_tokenId",
				"type": "uint256"
			}
		],
		"name": "tokenURI",
		"outputs": [
			{
				"internalType": "string",
				"name": "",
				"type": "string"
			}
		],
		"stateMutability": "view",
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


