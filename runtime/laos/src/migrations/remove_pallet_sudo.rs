use crate::{Runtime, Weight};
use frame_support::{migration, traits::OnRuntimeUpgrade};

pub struct Migration;

impl OnRuntimeUpgrade for Migration {
	fn on_runtime_upgrade() -> Weight {
		let result = migration::clear_storage_prefix(b"Sudo", b"Key", &[], None, None);
		log::info!("remove pallet_sudo migration result: {:?}", result.deconstruct());
		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(0, 1)
	}
}
