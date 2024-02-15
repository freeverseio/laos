# DPoS Pallet for Parachain Staking

This pallet is forked from `kilt` to avoid dependency on it. The exact commit hash of the fork is [`9b6b9da75496a4ab1a15a114679b6a942d4e73a1`](https://github.com/KILTprotocol/kilt-node/tree/9b6b9da75496a4ab1a15a114679b6a942d4e73a1);

## Collator Misconduct Policy

In a Proof of Stake (PoS) system, stakeholders can become block producers, termed as collators, and receive rewards for successfully generated blocks. Stakeholders also have the option to delegate their stakes to a collator, sharing in the rewards for each block authored by the chosen collator.

However, should a collator exhibit any form of misconduct—such as going offline, failing to validate, or producing invalid blocks—they will forfeit their rewards. By extension, delegators aligned with the non-compliant collator will also be affected, as they will not receive their expected rewards.

In response to such events, the governing body retains the right to decommission any collator found to be in violation of operational standards.