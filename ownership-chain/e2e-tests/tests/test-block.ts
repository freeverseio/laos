import { expect } from "chai";
import { GAS_LIMIT,  } from "./config";
import { describeWithExistingNode, customRequest } from "./util";

describeWithExistingNode("Frontier RPC (Block)", (context) => {
    it("should return genesis block by number", async function () {

        const block = await context.web3.eth.getBlock(0);
        expect(block).to.include({
            author: "0x0000000000000000000000000000000000000000",
            difficulty: "0",
            extraData: "0x",
            gasLimit: GAS_LIMIT,
            gasUsed: 0,
            logsBloom:
                "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            miner: "0x0000000000000000000000000000000000000000",
            number: 0,
            receiptsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            size: 505,
            timestamp: 0,
            totalDifficulty: "0",
        });

        expect(block.nonce).to.eql("0x0000000000000000");
        expect(block.hash).to.be.a("string").lengthOf(66);
        expect(block.parentHash).to.be.a("string").lengthOf(66);
        expect(block.timestamp).to.be.a("number");
    });
});
