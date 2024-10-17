import BN from "bn.js";
import { assert, expect } from "chai";
import { step } from "mocha-steps";

import { ASSET_HUB_PARA_ID, CHAIN_ID, FAITH, FAITH_PRIVATE_KEY, LAOS_PARA_ID, RUNTIME_IMPL_VERSION, RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION } from "./config";
import { customRequest, describeWithExistingNode } from "./util";
import { Keyring } from "@polkadot/api";

describeWithExistingNode("Asset Hub (Create Foreign Asset)", (context) => {
    step("HRMP channels between Asset Hub and LAOS are open", async function () {
        const laosToAssetHubChannel = await context.networks.relaychain.query.hrmp.hrmpChannels({
            sender: LAOS_PARA_ID,
            recipient: ASSET_HUB_PARA_ID,
        });
        expect(laosToAssetHubChannel.isEmpty).to.be.false;
        const assetHubToLaosChannel = await context.networks.relaychain.query.hrmp.hrmpChannels({
            sender: ASSET_HUB_PARA_ID,
            recipient: LAOS_PARA_ID,
        });
        expect(assetHubToLaosChannel.isEmpty).to.be.false;
    });
    // step("", async function () {
    //     const api = await context.polkadot.laos;
    //     const destination = api.createType('XcmVersionedLocation', {
    //         V2: {
    //             parents: '1',
    //             interior: {
    //                 X1: { Parachain: ASSET_HUB_PARACHAIN_ID },
    //             },
    //         },
    //     });

    //     let address = 'EGVQCe73TpFyAZx5uKfE1222XfkT3BSKozjgcqzLBnc5eYo';
    //     let accountId = api.createType('AccountId', address);
    //     console.log(accountId.toHex())

    //     const beneficiary = api.createType('XcmVersionedLocation', {
    //         V2: {
    //             parents: '0',
    //             interior: {
    //                 X1: {
    //                     AccountKey20: { // TODO
    //                         network: 'Any',
    //                         id: accountId.toHex(),
    //                     },
    //                 },
    //             },
    //         },
    //     });

    //     // 1 KSM = 10^12, this is .1 KSM
    //     const amount = 100000000000;
    //     const assets = api.createType('XcmVersionedAssets', {
    //         V2: [
    //             {
    //                 id: {
    //                     Concrete: {
    //                         parents: 0,
    //                         interior: {
    //                             Here: '',
    //                         },
    //                     },
    //                 },
    //                 fun: {
    //                     Fungible: amount,
    //                 },
    //             },
    //         ],
    //     });
    //     const fee_asset_item = '0';
    //     const weight_limit = 'Unlimited';

    //     const call = api.tx.polkadotXcm.limitedTeleportAssets(
    //         destination,
    //         beneficiary,
    //         assets,
    //         fee_asset_item,
    //         weight_limit
    //     );

    //     const faith = new Keyring({ type: "ethereum" }).addFromUri(FAITH_PRIVATE_KEY);
    //     const unsubscribe = await call.signAndSend(
    //         faith,
    //         ( status ) => {
    //             console.log(`Current status is ${status}`);

    //             // if (status.isInBlock) {
    //             //     console.log(`Transaction included at blockHash ${status.asInBlock}`);
    //             // } else if (status.isFinalized) {
    //             //     console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
    //             //     unsubscribe();
    //             // }
    //         }
    //     );
    // });

});
