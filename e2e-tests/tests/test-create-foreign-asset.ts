import BN from "bn.js";
import { assert, expect } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY, ASSET_HUB_PARA_ID, CHAIN_ID, FAITH, FAITH_PRIVATE_KEY, LAOS_PARA_ID, RUNTIME_IMPL_VERSION, RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION } from "./config";
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

    step("", async function () {
        const apiLaos = await context.networks.laos;
        const apiAssetHub = await context.networks.assetHub;
        const faith = new Keyring().addFromUri(FAITH_PRIVATE_KEY);
        const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

        const destination = apiLaos.createType('XcmVersionedLocation', {
            V3: {
                parents: '1',
                interior: {
                    X1: { Parachain: ASSET_HUB_PARA_ID },
                },
            },
        });
        
        let accountId = apiAssetHub.createType('AccountId', faith.address);
        const beneficiary = apiLaos.createType('XcmVersionedLocation', {
            V2: {
                parents: '0',
                interior: {
                    X1: {
                        AccountId32: {
                            network: 'Any',
                            id: accountId.toHex(),
                        },
                    },
                },
            },
        });

        // 1 KSM = 10^12, this is .1 KSM
        const amount = 100000000000;
        const assets = apiLaos.createType('XcmVersionedAssets', {
            V2: [
                {
                    id: {
                        Concrete: {
                            parents: 0,
                            interior: {
                                Here: '',
                            },
                        },
                    },
                    fun: {
                        Fungible: amount,
                    },
                },
            ],
        });
        const fee_asset_item = '0';
        const weight_limit = 'Unlimited';

        const call = apiLaos.tx.polkadotXcm.limitedTeleportAssets(
            destination,
            beneficiary,
            assets,
            fee_asset_item,
            weight_limit
        );

        // const unsubscribe = await call.signAndSend(
        //     alith,
        //     ( {status} ) => {
        //         console.log(`Current status is ${status}`);

        //         if (status.isInBlock) {
        //             console.log(`Transaction included at blockHash ${status.asInBlock}`);
        //         } else if (status.isFinalized) {
        //             console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
        //             unsubscribe();
        //         }
        //     }
        // );

        call.signAndSend(alith, (result) => {
            console.log(`RESULT =>>> ${result}`);
         })
            .catch((error: any) => {
                console.log("transaction failed", error);
            });
    });

});
