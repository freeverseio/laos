// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Evochain-to-OwnershipParachain headers sync entrypoint.
use crate::cli::bridge::{CliBridgeBase, RelayToRelayHeadersCliBridge};
use substrate_relay_helper::{
	finality::SubstrateFinalitySyncPipeline,
	finality_base::{engine::Grandpa as GrandpaFinalityEngine, SubstrateFinalityPipeline},
};
substrate_relay_helper::generate_submit_finality_proof_call_builder!(
	EvochainFinalityToOwnershipParachain,
	EvochainFinalityToOwnershipParachainCallBuilder,
	relay_laos_ownership_client::RuntimeCall::BridgeEvochainGrandpa,
	relay_laos_ownership_client::BridgeGrandpaCall::submit_finality_proof
);

/// Description of Evochain -> Rococo finalized headers bridge.
#[derive(Clone, Debug)]
pub struct EvochainFinalityToOwnershipParachain;

impl SubstrateFinalityPipeline for EvochainFinalityToOwnershipParachain {
	type SourceChain = relay_laos_evolution_client::Evochain;
	type TargetChain = relay_laos_ownership_client::OwnershipParachain;

	type FinalityEngine = GrandpaFinalityEngine<Self::SourceChain>;
}

impl SubstrateFinalitySyncPipeline for EvochainFinalityToOwnershipParachain {
	type SubmitFinalityProofCallBuilder = EvochainFinalityToOwnershipParachainCallBuilder;
}

//// `Evochain` to `OwnershipParachain`  bridge definition.
pub struct EvochainToOwnershipParachainCliBridge {}

impl CliBridgeBase for EvochainToOwnershipParachainCliBridge {
	type Source = relay_laos_evolution_client::Evochain;
	type Target = relay_laos_ownership_client::OwnershipParachain;
}

impl RelayToRelayHeadersCliBridge for EvochainToOwnershipParachainCliBridge {
	type Finality = EvochainFinalityToOwnershipParachain;
}
