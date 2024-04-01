// Copyright 2023-2024 Freverse.io
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

use hex_literal::hex;
use laos_primitives::AccountId;

pub const ALITH: [u8; 20] = hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac");
pub const BALTATHAR: [u8; 20] = hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0");
pub const CHARLETH: [u8; 20] = hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc");
pub const DOROTHY: [u8; 20] = hex!("773539d4Ac0e786233D90A233654ccEE26a613D9");
pub const ETHAN: [u8; 20] = hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB");
pub const FAITH: [u8; 20] = hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d");

/// Returns the accounts that are predefined in the runtime.
pub fn accounts() -> Vec<AccountId> {
	vec![
		ALITH.into(),
		BALTATHAR.into(),
		CHARLETH.into(),
		DOROTHY.into(),
		ETHAN.into(),
		FAITH.into(),
	]
}
