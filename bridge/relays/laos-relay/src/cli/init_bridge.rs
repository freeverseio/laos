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

use async_trait::async_trait;
use parity_scale_codec::Encode;

use crate::{
	bridges::{
		ownership_parachain_evochain::evochain_headers_to_ownership_parachain::EvochainToOwnershipParachainCliBridge,
		rococo_evochain::rococo_headers_to_evochain::RococoToEvochainCliBridge,
	},
	cli::{bridge::CliBridgeBase, chain_schema::*},
};
use bp_runtime::Chain as ChainBase;
use relay_substrate_client::{AccountKeyPairOf, Chain, UnsignedTransaction};
use sp_core::Pair;
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};
use substrate_relay_helper::finality_base::engine::{Engine, Grandpa as GrandpaFinalityEngine};

/// Initialize bridge pallet.
#[derive(StructOpt)]
pub struct InitBridge {
	/// A bridge instance to initialize.
	#[structopt(possible_values = InitBridgeName::VARIANTS, case_insensitive = true)]
	bridge: InitBridgeName,
	#[structopt(flatten)]
	source: SourceConnectionParams,
	#[structopt(flatten)]
	target: TargetConnectionParams,
	#[structopt(flatten)]
	target_sign: TargetSigningParams,
	/// Generates all required data, but does not submit extrinsic
	#[structopt(long)]
	dry_run: bool,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Bridge to initialize.
pub enum InitBridgeName {
	/// Evochain to Ownership Parachain bridge.
	EvochainToOwnershipParachain,
	/// Rococo to Evochain bridge.
	RococoToEvochain,
}

#[async_trait]
trait BridgeInitializer: CliBridgeBase
where
	<Self::Target as ChainBase>::AccountId: From<<AccountKeyPairOf<Self::Target> as Pair>::Public>,
{
	type Engine: Engine<Self::Source>;

	/// Get the encoded call to init the bridge.
	fn encode_init_bridge(
		init_data: <Self::Engine as Engine<Self::Source>>::InitializationData,
	) -> <Self::Target as Chain>::Call;

	/// Initialize the bridge.
	async fn init_bridge(data: InitBridge) -> anyhow::Result<()> {
		let source_client = data.source.into_client::<Self::Source>().await?;
		let target_client = data.target.into_client::<Self::Target>().await?;
		let target_sign = data.target_sign.to_keypair::<Self::Target>()?;
		let dry_run = data.dry_run;

		substrate_relay_helper::finality::initialize::initialize::<Self::Engine, _, _, _>(
			source_client,
			target_client.clone(),
			target_sign,
			move |transaction_nonce, initialization_data| {
				let call = Self::encode_init_bridge(initialization_data);
				log::info!(
					target: "bridge",
					"Initialize bridge call encoded as hex string: {:?}",
					format!("0x{}", hex::encode(call.encode()))
				);
				Ok(UnsignedTransaction::new(call.into(), transaction_nonce))
			},
			dry_run,
		)
		.await;

		Ok(())
	}
}

impl BridgeInitializer for EvochainToOwnershipParachainCliBridge {
	type Engine = GrandpaFinalityEngine<Self::Source>;

	fn encode_init_bridge(
		init_data: <Self::Engine as Engine<Self::Source>>::InitializationData,
	) -> <Self::Target as Chain>::Call {
		type RuntimeCall = relay_laos_ownership_client::RuntimeCall;
		type BridgeGrandpaCall = relay_laos_ownership_client::BridgeGrandpaCall;
		type SudoCall = relay_laos_ownership_client::SudoCall;

		let initialize_call: RuntimeCall =
			RuntimeCall::BridgeEvochainGrandpa(BridgeGrandpaCall::initialize { init_data });

		RuntimeCall::Sudo(SudoCall::sudo { call: Box::new(initialize_call) })
	}
}

impl BridgeInitializer for RococoToEvochainCliBridge {
	type Engine = GrandpaFinalityEngine<Self::Source>;
	fn encode_init_bridge(
		init_data: <Self::Engine as Engine<Self::Source>>::InitializationData,
	) -> <Self::Target as Chain>::Call {
		let initialize_call = laos_evolution_runtime::BridgeRococoGrandpaCall::<
			laos_evolution_runtime::Runtime,
			(),
		>::initialize {
			init_data,
		};

		relay_laos_evolution_client::RuntimeCall::Sudo(pallet_sudo::Call::<
			laos_evolution_runtime::Runtime,
		>::sudo {
			call: Box::new(initialize_call.into()),
		})
	}
}

impl InitBridge {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		match self.bridge {
			InitBridgeName::EvochainToOwnershipParachain =>
				EvochainToOwnershipParachainCliBridge::init_bridge(self),
			InitBridgeName::RococoToEvochain => RococoToEvochainCliBridge::init_bridge(self),
		}
		.await
	}
}
