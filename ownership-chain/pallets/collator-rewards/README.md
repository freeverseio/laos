## Collator Rewards Pallet

This pallet is responsible for distributing block rewards to collators. It uses `EventHandler` hook of `pallet_authorship` to trigger reward distribution in each block. Whenever a new block production round starts, `Authorship` pallet calls every type that implements `EventHandler` hook.

Reward distribution is infallible, i.e if there is not enough tokens in the `CommunityIncentivesAccount` to distribute, then the reward is not distributed. Once the account is funded, the reward distribution will resume.

### Configuration

- `CommunityIncentivesAccount` - an account that distributes block rewards.
- `RewardPerBlock` - amount of tokens that are distributed per block.
