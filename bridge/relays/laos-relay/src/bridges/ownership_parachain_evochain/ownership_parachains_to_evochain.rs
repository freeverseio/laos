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

//! Rialto-to-Millau parachains sync entrypoint.

use crate::cli::bridge::{CliBridgeBase, ParachainToRelayHeadersCliBridge};
use relay_laos_evolution_client::Evochain;
use relay_laos_ownership_client::OwnershipParachain;
use relay_rococo_client::Rococo;
use substrate_relay_helper::parachains::{
    DirectSubmitParachainHeadsCallBuilder, SubstrateParachainsPipeline,
};

/// Rialto-to-Millau parachains sync description.
#[derive(Clone, Debug)]
pub struct OwnershipParachainsToEvochain;

impl SubstrateParachainsPipeline for OwnershipParachainsToEvochain {
    type SourceParachain = OwnershipParachain;
    type SourceRelayChain = Rococo;
    type TargetChain = Evochain;

    type SubmitParachainHeadsCallBuilder =
        RococoParachainsToEvochainSubmitParachainHeadsCallBuilder;
}

/// `submit_parachain_heads` call builder for Rialto-to-Millau parachains sync pipeline.
pub type RococoParachainsToEvochainSubmitParachainHeadsCallBuilder =
    DirectSubmitParachainHeadsCallBuilder<
        OwnershipParachainsToEvochain,
        laos_evolution_runtime::Runtime,
        laos_evolution_runtime::WithRococoParachainsInstance,
    >;

/// `RialtoParachain` to `Millau` bridge definition.
pub struct RialtoParachainToMillauCliBridge {}

impl CliBridgeBase for RialtoParachainToMillauCliBridge {
    type Source = OwnershipParachain;
    type Target = Evochain;
}

impl ParachainToRelayHeadersCliBridge for RialtoParachainToMillauCliBridge {
    type SourceRelay = Rococo;
    type ParachainFinality = OwnershipParachainsToEvochain;
    type RelayFinality =
        crate::bridges::rococo_evochain::rococo_headers_to_evochain::RococoFinalityToEvochain;
}
