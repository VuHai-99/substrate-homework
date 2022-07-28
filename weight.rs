
//! Autogenerated weights for `pallet_kitties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-28, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet-kitties
// --extrinsic
// *
// --repeat
// 20
// --output
// weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_kitties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_kitties::WeightInfo for WeightInfo<T> {
	// Storage: KittiesModule Nonce (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: KittiesModule OwnerDetail (r:1 w:1)
	// Storage: KittiesModule KittyDetail (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: KittiesModule Kitties (r:1 w:1)
	fn create_kitty() -> Weight {
		(47_107_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: KittiesModule OwnerDetail (r:2 w:2)
	// Storage: KittiesModule KittyDetail (r:1 w:0)
	fn transfer() -> Weight {
		(37_404_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}