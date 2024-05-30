import { ApiPromise, WsProvider } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";
import fs from "fs";
import { decodeAddress } from "@polkadot/keyring";

const MAPPING_FILE = "bifrost_to_polkadot.md";
const POLKADOT_PREFIX = 0;
const BIFROST_PREFIX = 6;

async function main() {
  const provider = new WsProvider("wss://rpc.polkadot.io");
  const api = await ApiPromise.create({ provider });

  try {
    console.log('Parsing Bifrost contributions and creating map to Polkadot addresses...');
    const bifrostContributions = parseBifrostContributorsFile("bifrost_contributors");
    exportMapToFile(bifrostContributions, MAPPING_FILE);
  } catch (error) {
    console.error(error);
  } finally {
    await api.disconnect();
    process.exit(0);
  }
}

const exportMapToFile = (map: Map<string, string>, filename: string) => {
  const header ="| Bifrost Address   | Polkadot Address  |\n|----------|-------------|\n";
  const lines = Array.from(map.entries()).map(([addressBifrost, addressPolka]) => `|${addressBifrost}|${addressPolka}|`);
  fs.writeFileSync(filename, header + lines.join("\n"), "utf8");
  console.log(`...${filename} has been generated with ${map.size} entries.`);
};

const parseBifrostContributorsFile = (filename: string): Map<string, string> => {
  const contributionsFromFile = fs.readFileSync(filename, "utf8").trim().split("\n");
  const result = new Map<string, string>();

  contributionsFromFile.forEach(contribution => {
    const [contributor, amountStr] = contribution.split(",");
    const contributorPubKey = decodeAddress(contributor, false, BIFROST_PREFIX);
    const contributorPolkadotAddress = encodeAddress(contributorPubKey, POLKADOT_PREFIX);
    result.set(contributor, contributorPolkadotAddress);
  });
  return result;
};

main().catch(error => {
  console.error(error);
  process.exit(1);
});
