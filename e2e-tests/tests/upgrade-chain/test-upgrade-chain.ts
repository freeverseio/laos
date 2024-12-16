import { expect } from "chai";
import path from "path";
import { existsSync, readFileSync } from "fs";
import { step } from "mocha-steps";
import { describeWithExistingNode } from "@utils/setups";
import { CHOPSTICKS_LAOS_NODE_IP, RUNTIME_SPEC_VERSION, TARGET_PATH } from "@utils/constants";
import { sendTxAndWaitForFinalization } from "@utils/transactions";

describeWithExistingNode(
	"Runtime upgrade",
	function () {
		step("Runtime spec increases", async function () {
			const runtimePath = path.join(
				TARGET_PATH,
				"release/wbuild/laos-runtime/laos_runtime.compact.compressed.wasm"
			);

			if (!existsSync(runtimePath)) {
				throw new Error(`Unable to find LAOS wasm at ${runtimePath}`);
			}

			const wasmFile = readFileSync(runtimePath).toString("hex");

			const liveSpecVersion = this.chains.laos.consts.system.version.specVersion.toNumber();

			// The runtime version in LAOS is at smaller than the development version
			expect(
				liveSpecVersion < RUNTIME_SPEC_VERSION,
				"developed runtime version is not greater than the live chain version"
			).to.be.true;

			const tx = this.chains.laos.tx.sudo.sudo(this.chains.laos.tx.system.setCode(`0x${wasmFile}`));

			await sendTxAndWaitForFinalization(this.chains.laos, tx, this.ethereumPairs.alith);

			// Advance some blocks so the upgrade takes place
			await this.wsProvider.send("dev_newBlock", [{ count: 5 }]);

			const updatedSpecVersion = this.chains.laos.consts.system.version.specVersion.toNumber();

			expect(updatedSpecVersion === RUNTIME_SPEC_VERSION, "Runtime version wasn't upgraded").to.be.true;
		});
	},
	// Override LAOS node ip as this test is run with chopsticks
	CHOPSTICKS_LAOS_NODE_IP
);
