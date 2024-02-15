# DPoS Pallet for Parachain Staking

This pallet is forked from `kilt` to avoid dependency on it. The exact commit hash of the fork is [`9b6b9da75496a4ab1a15a114679b6a942d4e73a1`](https://github.com/KILTprotocol/kilt-node/tree/9b6b9da75496a4ab1a15a114679b6a942d4e73a1);

## Collator misbehaviour

In Proof of Stake (POS) systems, one can become a block producer (_collator_ in our case) based on their stake and earn rewards for producing blocks. Additionally, you can delegate or nominate your funds to one _collator_ and earn rewards for each authored block by that collator.

If the collator goes offline or misbehaves (fails to provide proof of validity, produces an invalid block, etc.), it will not be rewarded. Consequently, the delegators of that collator will not receive any rewards either.  

The governance could decide to offboard collators that misbehave.