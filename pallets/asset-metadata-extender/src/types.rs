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

//! Types used in the pallet
use sp_runtime::BoundedVec;

/// Wrapper around `BoundedVec` for `TokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// Wrapper around `BoundedVec` for `UniversalLocation`
pub type UniversalLocationOf<T> = BoundedVec<u8, <T as crate::Config>::MaxUniversalLocationLength>;

/// Serves as a position identifier for elements in a collection, facilitating iteration
/// and element access. It corresponds to the element count, ranging from 0 to N-1 in a collection
/// of size N.
pub type Index = u32;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
