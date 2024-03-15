use crate::{Runtime, RuntimeEvent};
use frame_support::parameter_types;
use pallet_ethereum::PostLogContent;

parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
	pub const ExtraDataLength: u32 = 30;
}

impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ExtraDataLength;
}
