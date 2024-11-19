import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { step } from "mocha-steps";
import { describeWithExistingNode, sendTxAndWaitForFinalization, waitForEventChopsticks } from "./util";

// Use chai-as-promised
chai.use(chaiAsPromised);

describeWithExistingNode(
    "Valid tx",
    function () {
        const newBlockMethod = "dev_newBlock"

        step("should create valid tx when new round starts", async function () {
            await this.context.providers.laos.send(newBlockMethod, [{ count: 3 }]);
            const roundLength = 10;
            const event = await waitForEventChopsticks(this.context.networks.laos, ({ event }) => {
                return this.context.networks.laos.events.parachainStaking.NewRound.is(event);
            }, this.context.providers.laos, roundLength);
            
            await this.context.providers.laos.send(newBlockMethod, [{ count: roundLength - 1 }]);
            const remarkTx = this.context.networks.laos.tx.system.remarkWithEvent("Hello, world!");
            await sendTxAndWaitForFinalization(this.context.networks.laos, remarkTx, this.ethereumPairs.alith);
        });

    },
    true
);
