/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
#[cfg(not(test))]
pub type XcmRouter = staging_xcm_builder::WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<crate::ParachainSystem, (), ()>,
	// ..and XCMP to communicate with the sibling chains.
	crate::XcmpQueue,
)>;

/// Use different router in `xcm-simulator` tests.
#[cfg(test)]
pub type XcmRouter = crate::tests::ParachainXcmRouter<crate::ParachainInfo>;
