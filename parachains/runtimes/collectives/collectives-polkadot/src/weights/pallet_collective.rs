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

//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: AllianceMotion Members (r:1 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:100 w:100)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `15691 + m * (1967 ±23) + p * (4332 ±23)`
		// Minimum execution time: 16_795_000 picoseconds.
		Weight::from_parts(16_942_000, 0)
			.saturating_add(Weight::from_parts(0, 15691))
			// Standard Error: 70_459
			.saturating_add(Weight::from_parts(5_607_397, 0).saturating_mul(m.into()))
			// Standard Error: 70_459
			.saturating_add(Weight::from_parts(8_290_115, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4332).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `1518 + m * (32 ±0)`
		// Minimum execution time: 15_803_000 picoseconds.
		Weight::from_parts(15_071_031, 0)
			.saturating_add(Weight::from_parts(0, 1518))
			// Standard Error: 21
			.saturating_add(Weight::from_parts(1_253, 0).saturating_mul(b.into()))
			// Standard Error: 219
			.saturating_add(Weight::from_parts(13_241, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:0)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `3498 + m * (32 ±0)`
		// Minimum execution time: 18_645_000 picoseconds.
		Weight::from_parts(17_717_343, 0)
			.saturating_add(Weight::from_parts(0, 3498))
			// Standard Error: 21
			.saturating_add(Weight::from_parts(1_471, 0).saturating_mul(b.into()))
			// Standard Error: 218
			.saturating_add(Weight::from_parts(22_238, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalCount (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `3714 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 24_537_000 picoseconds.
		Weight::from_parts(26_471_602, 0)
			.saturating_add(Weight::from_parts(0, 3714))
			// Standard Error: 79
			.saturating_add(Weight::from_parts(2_604, 0).saturating_mul(b.into()))
			// Standard Error: 829
			.saturating_add(Weight::from_parts(20_480, 0).saturating_mul(m.into()))
			// Standard Error: 818
			.saturating_add(Weight::from_parts(133_262, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `771 + m * (64 ±0)`
		//  Estimated: `4235 + m * (64 ±0)`
		// Minimum execution time: 23_195_000 picoseconds.
		Weight::from_parts(23_635_543, 0)
			.saturating_add(Weight::from_parts(0, 4235))
			// Standard Error: 399
			.saturating_add(Weight::from_parts(45_089, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3805 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 27_925_000 picoseconds.
		Weight::from_parts(29_021_838, 0)
			.saturating_add(Weight::from_parts(0, 3805))
			// Standard Error: 582
			.saturating_add(Weight::from_parts(19_351, 0).saturating_mul(m.into()))
			// Standard Error: 567
			.saturating_add(Weight::from_parts(120_374, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `662 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `3979 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 39_972_000 picoseconds.
		Weight::from_parts(40_831_166, 0)
			.saturating_add(Weight::from_parts(0, 3979))
			// Standard Error: 78
			.saturating_add(Weight::from_parts(1_568, 0).saturating_mul(b.into()))
			// Standard Error: 825
			.saturating_add(Weight::from_parts(16_454, 0).saturating_mul(m.into()))
			// Standard Error: 804
			.saturating_add(Weight::from_parts(132_248, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `458 + m * (48 ±0) + p * (36 ±0)`
		//  Estimated: `3898 + m * (49 ±0) + p * (36 ±0)`
		// Minimum execution time: 29_766_000 picoseconds.
		Weight::from_parts(31_292_444, 0)
			.saturating_add(Weight::from_parts(0, 3898))
			// Standard Error: 691
			.saturating_add(Weight::from_parts(19_285, 0).saturating_mul(m.into()))
			// Standard Error: 674
			.saturating_add(Weight::from_parts(123_923, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `682 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `3999 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 42_588_000 picoseconds.
		Weight::from_parts(43_162_079, 0)
			.saturating_add(Weight::from_parts(0, 3999))
			// Standard Error: 86
			.saturating_add(Weight::from_parts(1_451, 0).saturating_mul(b.into()))
			// Standard Error: 914
			.saturating_add(Weight::from_parts(18_855, 0).saturating_mul(m.into()))
			// Standard Error: 891
			.saturating_add(Weight::from_parts(136_799, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189 + p * (32 ±0)`
		//  Estimated: `1674 + p * (32 ±0)`
		// Minimum execution time: 14_465_000 picoseconds.
		Weight::from_parts(16_635_197, 0)
			.saturating_add(Weight::from_parts(0, 1674))
			// Standard Error: 1_082
			.saturating_add(Weight::from_parts(117_250, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}