// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_scheduler
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	/// Storage: Scheduler IncompleteSince (r:1 w:1)
	/// Proof: Scheduler IncompleteSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_agendas_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `1489`
		// Minimum execution time: 3_399_000 picoseconds.
		Weight::from_parts(3_512_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 200]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + s * (177 ±0)`
		//  Estimated: `159279`
		// Minimum execution time: 2_846_000 picoseconds.
		Weight::from_parts(4_715_748, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			// Standard Error: 2_743
			.saturating_add(Weight::from_parts(971_622, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_940_000 picoseconds.
		Weight::from_parts(6_087_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `179 + s * (1 ±0)`
		//  Estimated: `3644 + s * (1 ±0)`
		// Minimum execution time: 20_238_000 picoseconds.
		Weight::from_parts(20_509_000, 0)
			.saturating_add(Weight::from_parts(0, 3644))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(1_271, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
	}
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn service_task_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_274_000 picoseconds.
		Weight::from_parts(7_466_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_886_000 picoseconds.
		Weight::from_parts(6_035_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_994_000 picoseconds.
		Weight::from_parts(3_101_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_983_000 picoseconds.
		Weight::from_parts(3_095_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 199]`.
	fn schedule(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + s * (177 ±0)`
		//  Estimated: `159279`
		// Minimum execution time: 12_851_000 picoseconds.
		Weight::from_parts(14_995_788, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			// Standard Error: 3_056
			.saturating_add(Weight::from_parts(980_018, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 200]`.
	fn cancel(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + s * (177 ±0)`
		//  Estimated: `159279`
		// Minimum execution time: 17_805_000 picoseconds.
		Weight::from_parts(16_078_257, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			// Standard Error: 3_216
			.saturating_add(Weight::from_parts(1_767_117, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 199]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `468 + s * (179 ±0)`
		//  Estimated: `159279`
		// Minimum execution time: 16_210_000 picoseconds.
		Weight::from_parts(20_557_445, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			// Standard Error: 3_292
			.saturating_add(Weight::from_parts(992_281, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 200]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509 + s * (179 ±0)`
		//  Estimated: `159279`
		// Minimum execution time: 19_976_000 picoseconds.
		Weight::from_parts(19_613_223, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			// Standard Error: 2_988
			.saturating_add(Weight::from_parts(1_765_202, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
