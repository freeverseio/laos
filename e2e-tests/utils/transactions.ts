import { ApiPromise } from "@polkadot/api";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { SubmittableResult } from "@polkadot/api/submittable";
import { KeyringPair } from "@polkadot/keyring/types";
import { checkEventInBlock } from "@utils/blocks";

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
 * Sends a tx in assetHub chain and waits for its finality.
 *
 * NOTE:
 *  This line https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/session/src/lib.rs#L563 causes invalid transactions (txs)
 *  in "session.newSession" blocks (issue https://github.com/paritytech/polkadot-sdk/issues/184). When session.newSession is emitted in Rococo,
 *  txs sent to Rococo or AssetHub are rejected, causing sendTxAndWaitForFinalization to fail. Use this function to "dodge" session.newSession blocks.
 *
 * @param {ApiPromise} apiRelay - The ApiPromise to interact with Rococo.
 * @param {SubmittableExtrinsic<"promise">} tx - The tx to submit.
 * @param {KeyringPair} signer - The KeyRingPair used to sign the tx.
 * @param {ApiPromise} [apiAssetHub] - The ApiPromise to interact with AssetHub if needed.
 * @returns {Promise<string>} - A promise that resolves in the hash of the finalized block where the tx was included; rejects if the tx isn't valid or the execution resulted in a dispatch error.
 */
export async function sendTxAndWaitForFinalizationRococo(
	apiRelay: ApiPromise,
	tx: SubmittableExtrinsic<"promise">,
	signer: KeyringPair,
	apiAssetHub?: ApiPromise
): Promise<string> {
	const eventPromise = new Promise<void>(async (resolve) => {
		const unsub = await apiRelay.rpc.chain.subscribeFinalizedHeads(async (lastHeader) => {
			const event = await checkEventInBlock(
				apiRelay,
				({ event }) => apiRelay.events.session.NewSession.is(event),
				lastHeader.hash.toString(),
				false
			);
			if (event) {
				unsub();
				resolve();
			}
		});
	});

	await eventPromise;
	return apiAssetHub
		? sendTxAndWaitForFinalization(apiAssetHub, tx, signer)
		: sendTxAndWaitForFinalization(apiRelay, tx, signer);
}
