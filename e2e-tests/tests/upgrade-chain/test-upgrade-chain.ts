import { expect } from "chai";
import path from "path";
import { existsSync, readFileSync } from "fs";
import { step } from "mocha-steps";
import { describeWithExistingNode } from "@utils/setups";
import {
	CHOPSTICKS_LAOS_NODE_IP,
	CHOPSTICKS_POLKADOT_IP,
	LAOS_PARA_ID,
	RUNTIME_SPEC_VERSION,
	TARGET_PATH,
} from "@utils/constants";
import { sendTxAndWaitForFinalization } from "@utils/transactions";
import { checkEventInBlock } from "@utils/blocks";

describeWithExistingNode(
	"Runtime upgrade",
	function () {
		step("Runtime upgrade is performed if possible", async function () {
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

			const upgradeCooldowns = await this.chains.polkadot.query.paras.upgradeCooldowns();

			// If LAOS cannot be upgraded cause the cooldown is active (last upgrade's cooldown hasn't beeen completed)
			// we check that the upgrade is rejected. Otherwise, the upgrade goes on
			let upgradeForbidden = false;
			upgradeCooldowns.entries().forEach((entrie) => {
				const paraID = entrie[1][0];
				if (paraID.toNumber() === LAOS_PARA_ID) {
					upgradeForbidden = true;
				}
			});
			if (upgradeForbidden) {
				const finalizedBlock = await sendTxAndWaitForFinalization(
					this.chains.laos,
					tx,
					this.ethereumPairs.alith
				);
				const event = await checkEventInBlock(
					this.chains.laos,
					({ event }) => this.chains.laos.events.sudo.Sudid.is(event),
					finalizedBlock
				);

				const { index, error } = event.event.data.toJSON()[0]["err"]["module"];

				expect(index, "parachain System pallet index is 1").to.be.equal(1);
				expect(error, "parachain System ProhibitedByPolkadot error index is 0x01000000").to.be.equal(
					"0x01000000"
				);
			} else {
				await sendTxAndWaitForFinalization(this.chains.laos, tx, this.ethereumPairs.alith);

				// Advance a block so the upgrade takes place
				await this.wsProvider.send("dev_newBlock", [{ count: 1 }]);

				const updatedSpecVersion = this.chains.laos.consts.system.version.specVersion.toNumber();

				expect(updatedSpecVersion, "Runtime version wasn't upgraded").to.be.equal(RUNTIME_SPEC_VERSION);
			}
		});
	},
	// Override node IPs as this test is run with chopsticks
	CHOPSTICKS_LAOS_NODE_IP,
	CHOPSTICKS_POLKADOT_IP
);
