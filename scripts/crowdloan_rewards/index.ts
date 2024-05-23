import { ApiPromise, WsProvider } from "@polkadot/api";
import { StorageKey, Vec, Option } from "@polkadot/types";
import { BlockHash, StorageData } from "@polkadot/types/interfaces";
import { AnyTuple } from "@polkadot/types/types";
import { encodeAddress } from "@polkadot/util-crypto";
import fs from "fs";
import { hexToBn, BN } from "@polkadot/util";
import { decodeAddress } from "@polkadot/keyring";

const PARA_ID = 3370;
const CROWDLOAN_END_BLOCK = 20549306;
const BIFROST_POLKADOT_ADDRESS = "13YMK2eeopZtUNpeHnJ1Ws2HqMQG6Ts9PGCZYGyFbSYoZfcm";
const POLKADOT_PREFIX = 0;
const BIFROST_PREFIX = 6;
const REWARDS_FILE = "rewards.txt";
const CONTRIBUTIONS_FILE = "contributions.txt";
const HUMAN_READABLE_FILE = "crowdloan.md";
const LAOS_REWARD_PER_DOT = 100; // LAOS per DOT
const DOT_DECIMALS = 10;
const LAOS_DECIMALS = 18;
const DOT_TO_PLANCK: BN = new BN(10).pow(new BN(DOT_DECIMALS)); // 10^10
const LAOS_TO_WEI: BN = new BN(10).pow(new BN(LAOS_DECIMALS)); // 10^18

async function main() {
  const provider = new WsProvider("wss://rpc.polkadot.io");
  const api = await ApiPromise.create({ provider });

  try {
    const childKey = await api.derive.crowdloan.childKey(PARA_ID);
    const endCrowdloanBlockhash = await api.rpc.chain.getBlockHash(CROWDLOAN_END_BLOCK);
    console.log("Crowdloan end block: ", CROWDLOAN_END_BLOCK, "with hash:", endCrowdloanBlockhash.toString());
    const contributors = await fetchContributorsAt(api, childKey!, endCrowdloanBlockhash!);
    console.log("Contributors fetched:", contributors?.length);
    console.log("Getting the contributions for each contributor... (it may take a while)");

    const contributions = await addAmountToContributors(api, contributors!, endCrowdloanBlockhash!, childKey!);
    const totalContribution = Array.from(contributions.values()).reduce((acc, amount) => acc.add(amount), new BN(0));
    console.log("Total contribution:", totalContribution.toString(), "in DOT units");

    const bifrostContributionAmount = contributions.get(BIFROST_POLKADOT_ADDRESS)?.toString();
    console.log("Sovreign account of Bifrost contribution:", bifrostContributionAmount, "in DOT units");

    const bifrostContributions = parseBifrostContributorsFile("bifrost_contributors");
    const bifrostTotalContribution = Array.from(bifrostContributions.values()).reduce((acc, amount) => acc.add(amount), new BN(0));
    console.log("Bifrost contributions from file ./bifrost_contributors:", bifrostContributions.size, "for a total of", bifrostTotalContribution.toString(), "DOT units");  

    const hasBeenDeleted = contributions.delete(BIFROST_POLKADOT_ADDRESS);
    if (!hasBeenDeleted) {
      throw new Error("Bifrost contribution should have been deleted from the complete contribution list.");
    }

    const mergedContributions = mergeContributions(contributions, bifrostContributions);
    console.log("Polkadot contributions + Bifrost contributions:", mergedContributions.size);
    const totalMergedContributions = Array.from(mergedContributions.values()).reduce((acc, amount) => acc.add(amount), new BN(0));
    console.log("Total contributions (Polkadot + Bifrost):", totalMergedContributions.toString(), "DOT units");

    exportMapToFile(mergedContributions, CONTRIBUTIONS_FILE);
    console.log(`CONTRIBUTIONS HAVE BEEN SAVED TO ${CONTRIBUTIONS_FILE}.`);

    exportSummary(mergedContributions);
    console.log(`HUMAN READABLE SUMMARY HAS BEEN SAVED TO ${HUMAN_READABLE_FILE}.`);

    const rewardsMap = buildRewardsMap(mergedContributions);
    exportMapToFile(rewardsMap, REWARDS_FILE);
    const totalLaosRewards = Array.from(rewardsMap.values()).reduce((acc, amount) => acc.add(amount), new BN(0));
    console.log("Total LAOS rewards:", totalLaosRewards.toString(), "LAOS units");
    console.log("***************************************************************");
    console.log(`----> REWARDS HAVE BEEN SAVED TO ${REWARDS_FILE}. <----`);
    console.log("***************************************************************");
  } catch (error) {
    console.error(error);
  } finally {
    await api.disconnect();
    process.exit(0);
  }
}

function moveDecimalPointToLeft(bigNumber: BN, decimalPlaces: number): string {
  // If the length of bigNumberStr is less than or equal to decimalPlaces, prepend zeros
  // This is never exercised in our case, since the min DOT contribution was 5
  const paddedBigNumberStr = bigNumber.toString(10).padStart(decimalPlaces + 1, '0');

  const integerPart = paddedBigNumberStr.slice(0, -decimalPlaces);
  const decimalPart = paddedBigNumberStr.slice(-decimalPlaces);

  const trimmedDecimalPart = decimalPart.replace(/0+$/, ''); // remove trailing 0s
  return trimmedDecimalPart.length > 0 ? `${integerPart}.${trimmedDecimalPart}` : `${integerPart}`;
}

const exportSummary = (contributors: Map<string, BN>)  => {
  const lines: string[] = [];

  lines.push("| Contributor Address   | DOT contributed  | Entitled LAOS Rewards  |");
  lines.push("|----------|----------|-------------|");

  contributors.forEach((planck, address) => {
    const dot = moveDecimalPointToLeft(planck, DOT_DECIMALS);
    const laos = moveDecimalPointToLeft(planck, DOT_DECIMALS - 2); // LAOS rewards = 100x DOT contributed
    lines.push(`|${address}|${dot}|${laos}`);
  });

  fs.writeFileSync(HUMAN_READABLE_FILE, lines.join("\n"), "utf8");
  console.log(`${HUMAN_READABLE_FILE} has been generated with ${contributors.size} entries.`);
};

const buildRewardsMap = (contributors: Map<string, BN>): Map<string, BN> => {
  const result = new Map<string, BN>();

  contributors.forEach((amount, address) => {
    const laosDotDecimalRatio = LAOS_TO_WEI.div(DOT_TO_PLANCK); // 10**8
    const rewardAmount = amount.mul(laosDotDecimalRatio).muln(LAOS_REWARD_PER_DOT);
    result.set(address, rewardAmount);
  });

  return result;
};

const exportMapToFile = (map: Map<string, BN>, filename: string) => {
  const lines = Array.from(map.entries()).map(([address, amount]) => `|${address}|${amount.toString()}|`);
  fs.writeFileSync(filename, lines.join("\n"), "utf8");
  console.log(`${filename} has been generated with ${map.size} entries.`);
};

const mergeContributions = (map1: Map<string, BN>, map2: Map<string, BN>): Map<string, BN> => {
  const result = new Map<string, BN>(map1);

  map2.forEach((amount, address) => {
    if (result.has(address)) {
      const existingAmount = result.get(address) as BN;
      result.set(address, existingAmount.add(amount));
    } else {
      result.set(address, amount);
    }
  });

  return result;
};

const addAmountToContributors = async (
  api: ApiPromise,
  contributors: Vec<StorageKey<AnyTuple>>,
  at: BlockHash,
  childKey: string
): Promise<Map<string, BN>> => {
  const result = new Map<string, BN>();

  for (const contributor of contributors) {
    const contributionAmountScaleEncoded = await getContributionAmountFor(api, contributor.toString(), at, childKey);
    let contributionAmount = new BN(0);
    if (contributionAmountScaleEncoded) {
      contributionAmount = hexToBn(contributionAmountScaleEncoded.toHex(), { isLe: true });
    }
    const contributorSS58 = encodeAddress(contributor, POLKADOT_PREFIX);
    result.set(contributorSS58, contributionAmount);
  }

  return result;
};

const parseBifrostContributorsFile = (filename: string): Map<string, BN> => {
  const contributionsFromFile = fs.readFileSync(filename, "utf8").trim().split("\n");
  const result = new Map<string, BN>();

  contributionsFromFile.forEach(contribution => {
    const [contributor, amountStr] = contribution.split(",");
    const contributorPubKey = decodeAddress(contributor, false, BIFROST_PREFIX);
    const contributorPolkadotAddress = encodeAddress(contributorPubKey, POLKADOT_PREFIX);

    const amountBN = new BN((parseFloat(amountStr) * DOT_TO_PLANCK.toNumber()).toFixed(0));
    if (result.has(contributorPolkadotAddress)) {
      const increasedAmount = amountBN.add(result.get(contributorPolkadotAddress)!);
      result.set(contributorPolkadotAddress, increasedAmount);
    } else {
      result.set(contributorPolkadotAddress, amountBN);
    }
  });

  return result;
};

const getContributionAmountFor = async (
  api: ApiPromise,
  publicKey: string,
  at: BlockHash,
  childKey: string
): Promise<StorageData> => {
  const result: Option<StorageData> = await api.rpc.childstate.getStorage(childKey, publicKey, at);
  if (!result.isSome) {
    throw new Error(`No contribution amount found for publicKey: ${publicKey}`);
  }
  return result.unwrap();
};

const fetchContributorsAt = async (
  api: ApiPromise,
  childKey: string,
  at: BlockHash
): Promise<Vec<StorageKey> | undefined> => {
  try {
    return await api.rpc.childstate.getKeys(childKey, "0x", at);
  } catch (error) {
    console.error("Failed to fetch contributors:", error);
  }
  return undefined;
};

main().catch(error => {
  console.error(error);
  process.exit(1);
});
