//! Rococo-to-Evochain headers sync entrypoint.

use crate::cli::bridge::{CliBridgeBase, RelayToRelayHeadersCliBridge};
use substrate_relay_helper::{
	finality::{DirectSubmitGrandpaFinalityProofCallBuilder, SubstrateFinalitySyncPipeline},
	finality_base::{engine::Grandpa as GrandpaFinalityEngine, SubstrateFinalityPipeline},
};

/// Description of Rococo -> Evochain finalized headers bridge.
#[derive(Clone, Debug)]
pub struct RococoFinalityToEvochain;

impl SubstrateFinalityPipeline for RococoFinalityToEvochain {
	type SourceChain = relay_rococo_client::Rococo;
	type TargetChain = relay_laos_evolution_client::Evochain;

	type FinalityEngine = GrandpaFinalityEngine<Self::SourceChain>;
}

impl SubstrateFinalitySyncPipeline for RococoFinalityToEvochain {
	type SubmitFinalityProofCallBuilder =
		DirectSubmitGrandpaFinalityProofCallBuilder<Self, laos_evolution_runtime::Runtime, ()>;
}

/// `Rococo` to `Evochain` bridge definition.
pub struct RococoToEvochainCliBridge {}

impl CliBridgeBase for RococoToEvochainCliBridge {
	type Source = relay_rococo_client::Rococo;
	type Target = relay_laos_evolution_client::Evochain;
}

impl RelayToRelayHeadersCliBridge for RococoToEvochainCliBridge {
	type Finality = RococoFinalityToEvochain;
}
