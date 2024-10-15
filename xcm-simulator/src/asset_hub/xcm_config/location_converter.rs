use super::{super::AccountId, constants::RelayNetwork};
use polkadot_parachain_primitives::primitives::Sibling;
use xcm_builder::{AccountId32Aliases, DescribeAllTerminal, DescribeFamily, HashedDescription, ParentIsPreset, SiblingParachainConvertsVia};

type LocationToAccountId = (
	ParentIsPreset<AccountId>,
	SiblingParachainConvertsVia<Sibling, AccountId>,
	HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
	AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type LocationConverter = LocationToAccountId;
