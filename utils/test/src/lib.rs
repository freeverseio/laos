//! Contains commonly used test functions and types

/// Rolls to a block number by simulating the block production
///
/// ```rs
/// roll_one_block!(true);
/// ```
#[macro_export]
macro_rules! roll_one_block {
	($staking_enabled: expr) => {
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::reset_events();
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		if $staking_enabled {
			ParachainStaking::on_initialize(System::block_number());
		}
	};
}

/// Asserts that some events were never emitted.
///
/// # Example
///
/// ```
/// assert_no_events!();
/// ```
#[macro_export]
macro_rules! assert_no_events {
	() => {
		similar_asserts::assert_eq!(Vec::<Event<Test>>::new(), crate::mock::events())
	};
}

/// Asserts that emitted events match exactly the given input.
///
/// # Example
///
/// ```
/// assert_events_eq!(
/// 		Foo { x: 1, y: 2 },
/// 		Bar { value: "test" },
/// 		Baz { a: 10, b: 20 },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_eq {
	($event:expr) => {
		similar_asserts::assert_eq!(vec![$event], crate::mock::events());
	};
	($($events:expr,)+) => {
		similar_asserts::assert_eq!(vec![$($events,)+], crate::mock::events());
	};
}

/// Asserts that some emitted events match the given input.
///
/// # Example
///
/// ```
/// assert_events_emitted!(
/// 		Foo { x: 1, y: 2 },
/// 		Baz { a: 10, b: 20 },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_emitted {
	($event:expr) => {
		[$event].into_iter().for_each(|e| assert!(
			crate::mock::events().into_iter().find(|x| x == &e).is_some(),
			"Event {:?} was not found in events: \n{:#?}",
			e,
			crate::mock::events()
		));
	};
	($($events:expr,)+) => {
		[$($events,)+].into_iter().for_each(|e| assert!(
			crate::mock::events().into_iter().find(|x| x == &e).is_some(),
			"Event {:?} was not found in events: \n{:#?}",
			e,
			crate::mock::events()
		));
	};
}

/// Asserts that some events were never emitted.
///
/// # Example
///
/// ```
/// assert_events_not_emitted!(
/// 		Foo { x: 1, y: 2 },
/// 		Bar { value: "test" },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_not_emitted {
	($event:expr) => {
		[$event].into_iter().for_each(|e| assert!(
			crate::mock::events().into_iter().find(|x| x != &e).is_some(),
			"Event {:?} was unexpectedly found in events: \n{:#?}",
			e,
			crate::mock::events()
		));
	};
	($($events:expr,)+) => {
		[$($events,)+].into_iter().for_each(|e| assert!(
			crate::mock::events().into_iter().find(|x| x != &e).is_some(),
			"Event {:?} was unexpectedly found in events: \n{:#?}",
			e,
			crate::mock::events()
		));
	};
}

/// Asserts that the emitted events are exactly equal to the input patterns.
///
/// # Example
///
/// ```
/// assert_events_eq_match!(
/// 		Foo { x: 1, .. },
/// 		Bar { .. },
/// 		Baz { a: 10, b: 20 },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_eq_match {
	($index:expr;) => {
		assert_eq!(
			$index,
			crate::mock::events().len(),
			"Found {} extra event(s): \n{:#?}",
			crate::mock::events().len()-$index,
			crate::mock::events()
		);
	};
	($index:expr; $event:pat_param, $($events:pat_param,)*) => {
		assert!(
			matches!(
				crate::mock::events().get($index),
				Some($event),
			),
			"Event {:#?} was not found at index {}: \n{:#?}",
			stringify!($event),
			$index,
			crate::mock::events()
		);
		assert_events_eq_match!($index+1; $($events,)*);
	};
	($event:pat_param) => {
		assert_events_eq_match!(0; $event,);
	};
	($($events:pat_param,)+) => {
		assert_events_eq_match!(0; $($events,)+);
	};
}

/// Asserts that some emitted events match the input patterns.
///
/// # Example
///
/// ```
/// assert_events_emitted_match!(
/// 		Foo { x: 1, .. },
/// 		Baz { a: 10, b: 20 },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_emitted_match {
	($event:pat_param) => {
		assert!(
			crate::mock::events().into_iter().any(|x| matches!(x, $event)),
			"Event {:?} was not found in events: \n{:#?}",
			stringify!($event),
			crate::mock::events()
		);
	};
	($event:pat_param, $($events:pat_param,)+) => {
		assert_events_emitted_match!($event);
		$(
			assert_events_emitted_match!($events);
		)+
	};
}

/// Asserts that the input patterns match none of the emitted events.
///
/// # Example
///
/// ```
/// assert_events_not_emitted_match!(
/// 		Foo { x: 1, .. },
/// 		Baz { a: 10, b: 20 },
/// );
/// ```
#[macro_export]
macro_rules! assert_events_not_emitted_match {
	($event:pat_param) => {
		assert!(
			crate::mock::events().into_iter().any(|x| !matches!(x, $event)),
			"Event {:?} was unexpectedly found in events: \n{:#?}",
			stringify!($event),
			crate::mock::events()
		);
	};
	($event:pat_param, $($events:pat_param,)+) => {
		assert_events_not_emitted_match!($event);
		$(
			assert_events_not_emitted_match!($events);
		)+
	};
}
