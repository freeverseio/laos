import hre from "hardhat"
import { expect } from "chai";

describe("Deploy solidity contract", async function () {
    it("I can get the address", async function () {
        const storage = await hre.ethers.deployContract("Storage");
        await storage.waitForDeployment();
        expect(storage.target).to.be.properAddress;
    })
})
