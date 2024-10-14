pub use sandbox::*;

mod sandbox {
	use frame_support::{parameter_types, traits::ContainsPair};
	use sp_runtime::traits::Get;
	use xcm::latest::prelude::*;
	use xcm_simulator::{Asset, ParaId};

	pub struct IsForeignConcreteAsset<IsForeign>(sp_std::marker::PhantomData<IsForeign>);
	impl<IsForeign: ContainsPair<Location, Location>> ContainsPair<Asset, Location>
		for IsForeignConcreteAsset<IsForeign>
	{
		fn contains(asset: &Asset, origin: &Location) -> bool {
			log::trace!(target: "xcm::contains", "IsForeignConcreteAsset asset: {:?}, origin: {:?}", asset, origin);
			matches!(asset.id, ref id if IsForeign::contains(&id.0, origin))
		}
	}

	pub struct FromSiblingParachain<SelfParaId>(sp_std::marker::PhantomData<SelfParaId>);
	impl<SelfParaId: Get<ParaId>> ContainsPair<Location, Location>
		for FromSiblingParachain<SelfParaId>
	{
		fn contains(a: &Location, b: &Location) -> bool {
			// `a` needs to be from `b` at least.
			if !a.starts_with(&b) {
				return false;
			}

			// here we check if sibling
			match a.unpack() {
				(1, interior) =>
					matches!(interior.first(), Some(Parachain(sibling_para_id)) if sibling_para_id.ne(&u32::from(SelfParaId::get()))),
				_ => false,
			}
		}
	}

	parameter_types! {
		pub SelfParaId: ParaId = 1000.into();
	}

	pub type TrustedTeleporters = IsForeignConcreteAsset<FromSiblingParachain<SelfParaId>>;
}
