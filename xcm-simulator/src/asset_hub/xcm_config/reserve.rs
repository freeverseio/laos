pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	/// We don't trust any chain as a reserve.
	pub type TrustedReserves = ();
}
