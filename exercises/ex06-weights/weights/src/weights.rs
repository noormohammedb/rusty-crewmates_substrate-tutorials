
//! Autogenerated weights for `pallet_weight`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `TechTonic.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet-weight
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ../exercises/ex06-weights/weights/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use crate as pallet_weight;

/// Weight functions for `pallet_weight`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_weight::WeightInfo for WeightInfo<T> {
	// Storage: WeightModule VecDup (r:0 w:1)
	/// The range of component `s` is `[1, 10000]`.
	fn duplicate_and_store(s: u32, ) -> Weight {
		(3_275_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: WeightModule Data (r:0 w:1)
	fn store_maybe_hashed_true() -> Weight {
		(124_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: WeightModule Data (r:0 w:1)
	fn store_maybe_hashed_false() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
