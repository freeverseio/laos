// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use crate::{ParachainSystem, Runtime};

use frame_support::{parameter_types, weights::Weight};
use pallet_treasury::migration::cleanup_proposals::Migration as TreasuryMigration;

parameter_types! {
	/// Weight for balance unreservations
	pub BalanceUnreserveWeight: Weight = Weight::from_parts(18_890_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(<Runtime as frame_system::Config>::DbWeight::get().reads(1))
			.saturating_add(<Runtime as frame_system::Config>::DbWeight::get().writes(1));
}

pub type Migrations = (
	cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4<Runtime>,
	cumulus_pallet_xcmp_queue::migration::v5::MigrateV4ToV5<Runtime>,
	TreasuryMigration<Runtime, (), BalanceUnreserveWeight>,
);

impl cumulus_pallet_xcmp_queue::migration::v5::V5Config for Runtime {
	type ChannelList = ParachainSystem;
}
