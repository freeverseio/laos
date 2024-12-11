import chai, { expect } from "chai";
import path from "path";
import { existsSync } from "fs";
import { step } from "mocha-steps";
import { describeWithExistingNode } from "@utils/setups";
import { CHOPSTICKS_LAOS_NODE_IP, RUNTIME_SPEC_VERSION, TARGET_PATH } from "@utils/constants";

describeWithExistingNode(
	"Runtime upgrade",
	function () {
		step("Runtime spec increases", async function () {
			const runtime_path = path.join(
				TARGET_PATH,
				"release/wbuild/laos-runtime/laos_runtime.compact.compressed.wasm"
			);
			if (!existsSync(runtime_path)) {
				throw new Error(`Unable to find LAOS wasm at ${runtime_path}`);
			}
		});
	},
	// Override LAOS node ip as this test is run with chopsticks
	CHOPSTICKS_LAOS_NODE_IP
);
