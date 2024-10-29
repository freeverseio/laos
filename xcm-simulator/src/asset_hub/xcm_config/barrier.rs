pub use sandbox::*;

#[cfg(not(feature = "barrier"))]
mod sandbox {
	use frame_support::traits::Everything;
	use xcm_builder::AllowUnpaidExecutionFrom;

	pub type Barrier = AllowUnpaidExecutionFrom<Everything>;
}

#[cfg(feature = "barrier")]
mod sandbox {
	use frame_support::traits::{Contains, Everything};
	use xcm::prelude::*;
	use xcm_builder::{AllowExplicitUnpaidExecutionFrom, AllowTopLevelPaidExecutionFrom};

	pub struct ParentRelay;
	impl Contains<Location> for ParentRelay {
		fn contains(location: &Location) -> bool {
			matches!(location.unpack(), (1, []))
		}
	}

	pub type Barrier =
		(AllowTopLevelPaidExecutionFrom<Everything>, AllowExplicitUnpaidExecutionFrom<ParentRelay>);
}
