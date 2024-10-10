// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Parachain runtime mock.

mod configs;

pub use configs::xcm_config::ASSET_HUB_ID;
use frame_support::construct_runtime;

use crate::mock_msg_queue;

pub type AccountId = laos_primitives::AccountId;
pub type Balance = laos_primitives::Balance;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime
	{
		System: frame_system,
		ParachainInfo: parachain_info,
		Balances: pallet_balances,
		MsgQueue: mock_msg_queue,
		PolkadotXcm: pallet_xcm,
		CumulusXcm: cumulus_pallet_xcm,
	}
);
