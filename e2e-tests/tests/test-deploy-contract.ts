import { describeWithExistingNode } from "./util";
import { ALITH, ALITH_PRIVATE_KEY, BALANCES_ABI, BALANCES_BYTECODE, GAS_LIMIT, GAS_PRICE } from "./config";
import { expect } from "chai";

describeWithExistingNode("Frontier RPC (Deploying contract)", (context) => {
    it("", async function () {
        const contract = new context.web3.eth.Contract(BALANCES_ABI, null, {
            gasPrice: GAS_PRICE,
            gas: GAS_LIMIT,
        });
        context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
        
        const tokenName = "FutureLAOS";
        const tokenSymbol = "FLAOS";
        
        const deployOptions = {
            data: BALANCES_BYTECODE,
            arguments: [tokenName, tokenSymbol]
        };
        // const estimatedGas = await contract.deploy(deployOptions).estimateGas();
        // const gasPrice = (await context.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
        let nonce = await context.web3.eth.getTransactionCount(ALITH);
        const gasPrice = await context.web3.eth.getGasPrice();
        const estimatedGas = await contract.deploy(deployOptions).estimateGas({ gas: GAS_LIMIT * 2}); // en moonbeam 1861715
        console.log(estimatedGas);
        // const result = await contract.deploy(deployOptions).send({ from: ALITH, gas: 1557906, gasPrice, nonce: nonce++ }); // en moonbeam 1861715
        // expect(context.web3.utils.isAddress(result.options.address)).to.be.eq(true);
    })

});
