mod init_pallet;

use crate::Preimage;

pub type Migrations = init_pallet::InitializePallet<Preimage>;
