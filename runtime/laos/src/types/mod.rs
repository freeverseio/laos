mod account_id_to_h160;
mod to_author;
mod transaction_converter;

use super::{
	precompiles::FrontierPrecompiles, AccountId, AllPalletsWithSystem, Header, Runtime,
	RuntimeCall, Signature,
};
use sp_runtime::generic;

pub(crate) use account_id_to_h160::AccountIdToH160;
pub(crate) use to_author::ToAuthor;
pub use transaction_converter::TransactionConverter;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

pub type Precompiles = FrontierPrecompiles<Runtime>;

/// Unchecked extrinsic type as expected by this runtime.
pub(crate) type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub(crate) type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

/// The SignedExtension to the basic transaction logic.
type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
