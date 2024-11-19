import chai, { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import { VESTING_CONTRACT_ADDRESS, VESTING_ABI, UNIT, GAS_PRICE } from "./config";
import { describeWithExistingNode, sendTxAndWaitForFinalization, waitForConfirmations, waitForBlocks, waitForEvent } from "./util";

// Use chai-as-promised
chai.use(chaiAsPromised);

describeWithExistingNode(
    "Frontier RPC (Vesting)",
    function () {

        step("should create valid tx when new round starts", async function () {
            
            // avanzar un bloque
            let event;
            while (event == null) {

                await this.context.providers.laos.send("dev_newBlock", [{ count: 1 }]);
                event = await waitForEvent(
                    this.context.networks.laos,
                    ({ event }) => {
                        return this.context.networks.laos.events.parachainStaking.NewRound.is(event);
                    },
                    1
                );
            }
            console.log(event.toHuman())

            // expect(event).to.not.be.null;
            // ver si es un new round

        });

    },
    true
);
