pub use sandbox::*;

mod sandbox {
	use frame_support::{parameter_types, traits::EverythingBut};
	use xcm::prelude::*;
	use xcm_builder::{
		FungibleAdapter, FungiblesAdapter, IsConcrete, MatchedConvertedConcreteId, NoChecking,
		StartsWith,
	};
	use xcm_executor::traits::JustTry;

	use crate::asset_hub::{
		location_converter::LocationConverter, AccountId, Balance, Balances, ForeignAssets,
		PolkadotXcm,
	};

	/// AssetTransactor for handling the chain's native token.
	pub type FungibleTransactor = FungibleAdapter<
		// Use this implementation of the `fungible::*` traits.
		// `Balances` is the name given to the balances pallet in this particular example.
		// Any implementation of the traits would suffice.
		Balances,
		// This transactor deals with the native token of the Relay Chain.
		// This token is referenced by the Location of the Relay Chain relative to this chain
		// -- Location::parent().
		IsConcrete<ParentLocation>,
		// How to convert an XCM Location into a local account id.
		// This is also something that's configured in the XCM executor.
		LocationConverter,
		// The type for account ids, only needed because `fungible` is generic over it.
		AccountId,
		// Not tracking teleports.
		// This recipe only uses reserve asset transfers to handle the Relay Chain token.
		(),
	>;

	parameter_types! {
		pub ParentLocation: Location = Location::parent();
		pub LocalPrefix: Location = Location::here();
		pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	}

	/// Type that matches foreign assets.
	/// We do this by matching on all possible Locations and excluding the ones
	/// inside our local chain.
	pub type ForeignAssetsMatcher = MatchedConvertedConcreteId<
		Location,                               // Asset id.
		Balance,                                // Balance type.
		EverythingBut<StartsWith<LocalPrefix>>, // Location matcher.
		JustTry,                                // How to convert from Location to AssetId.
		JustTry,                                // How to convert from u128 to Balance.
	>;

	/// AssetTransactor for handling other parachains' native tokens.
	pub type ForeignFungiblesTransactor = FungiblesAdapter<
		// Use this implementation of the `fungibles::*` traits.
		// `Balances` is the name given to the balances pallet in this particular example.
		ForeignAssets,
		// This transactor deals with the native token of sibling parachains.
		ForeignAssetsMatcher,
		// How we convert from a Location to an account id.
		LocationConverter,
		// The `AccountId` type.
		AccountId,
		// Not tracking teleports since we only use reserve asset transfers.
		NoChecking,
		// The account for checking.
		CheckingAccount,
	>;

	pub type AssetTransactor = (FungibleTransactor, ForeignFungiblesTransactor);
}
