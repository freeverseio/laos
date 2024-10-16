import BN from "bn.js";
import { assert, expect } from "chai";
import { step } from "mocha-steps";

import { CHAIN_ID, FAITH, RUNTIME_IMPL_VERSION, RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION } from "./config";
import { customRequest, describeWithExistingNode } from "./util";

describeWithExistingNode("Asset Hub (Create Foreign Asset)", (context) => {
    step("", async function () {
        const api = await context.polkadot.assetHub;
        console.log(api)
    });

});
