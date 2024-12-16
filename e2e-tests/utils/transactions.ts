import { ApiPromise } from "@polkadot/api";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { SubmittableResult } from "@polkadot/api/submittable";
import { KeyringPair } from "@polkadot/keyring/types";
import Web3 from "web3";

import debug from "debug";
const debugTx = debug("transaction");

/**
 * Sends a tx in a specified chain and waits for its finality
 * @param {ApiPromise} api - The ApiPromise to interact with the chain.
 * @param {SubmittableExtrinsic<"promise">} tx - The tx to submit.
 * @param {KeyringPair} signer - The KeyRingPair used to sign the tx.
 * @returns {Promise<string>} - A promise that resolves in the hash of the finalized block where the tx was included; rejects if the tx isn't valid or the execution resulted in a dispatch error.
 */
export async function sendTxAndWaitForFinalization(
	api: ApiPromise,
	tx: SubmittableExtrinsic<"promise">,
	signer: KeyringPair
): Promise<string> {
	return new Promise(async (resolve, reject) => {
		try {
			const unsub = await tx.signAndSend(signer, (result: SubmittableResult) => {
				const { status, dispatchError } = result;
				debugTx("Tx status: ", status.type);

				if (dispatchError) {
					debugTx("Raw dispatch error:", dispatchError.toString());

					if (dispatchError.isModule) {
						const decoded = api.registry.findMetaError(dispatchError.asModule);
						const { section, name, docs } = decoded;
						debugTx(`Transaction failed with error: ${section}.${name}`);
						debugTx(`Error documentation: ${docs.join(" ")}`);
						reject(new Error(`${section}.${name}: ${docs.join(" ")}`));
					} else {
						debugTx(`Transaction failed with error: ${dispatchError.toString()}`);
						reject(new Error(dispatchError.toString()));
					}
					return;
				}

				if (status.isInBlock) {
					debugTx("Tx included at block hash: ", status.asInBlock.toHex());
				} else if (status.isFinalized) {
					debugTx("Tx finalized at block hash: ", status.asFinalized.toHex());
					unsub();
					resolve(status.asFinalized.toHex());
				} else if (status.isInvalid || status.isDropped || status.isUsurped || status.isFinalityTimeout) {
					unsub();
					reject(new Error("Tx won't be included in chain"));
				}
			});
		} catch (error) {
			debugTx("Error during transaction setup:", error);
			reject(error);
		}
	});
}

/**
 * Wait til a transaction sent by a EVM contract is included in a finalized block
 * @param {Web3} web3 - The web3.js provider
 * @param {ApiPromise} api - The ApiPromise used to interact with the chain
 * @param {string} txHash - The transaction hash
 */
export async function waitFinalizedEthereumTx(web3: Web3, api: ApiPromise, txHash: string) {
	try {
		while (true) {
			const receipt = await web3.eth.getTransactionReceipt(txHash);
			if (receipt && receipt.blockNumber) {
				const finalizedBlock = (
					await api.rpc.chain.getBlock(await api.rpc.chain.getFinalizedHead())
				).block.header.number.toNumber();
				if (finalizedBlock >= receipt.blockNumber) {
					return;
				}
			}
			// Polling to avoid querying the block numbers so frequently cause they aren't produced that fast
			setTimeout(() => {}, 2000);
		}
	} catch (error) {
		debugTx(`Error waiting for confirmations of transaction ${txHash}:`, error);
		throw error;
	}
}
