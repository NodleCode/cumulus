// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_bridge_assets_transfer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `covid`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemine-dev"), DB CACHE: 1024

// Executed Command:
// target/debug/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_bridge_assets_transfer
// --chain=statemine-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemine/src/weights/bridge_assets_transfer.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_assets_transfer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_assets_transfer::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BridgeAssetsTransfer Bridges (r:1 w:0)
	/// Proof: BridgeAssetsTransfer Bridges (max_values: None, max_size: Some(1899), added: 4374, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem RelevantMessagingState (r:1 w:0)
	/// Proof Skipped: ParachainSystem RelevantMessagingState (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmpQueue OutboundXcmpStatus (r:1 w:1)
	/// Proof Skipped: XcmpQueue OutboundXcmpStatus (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: XcmpQueue OutboundXcmpMessages (r:0 w:1)
	/// Proof Skipped: XcmpQueue OutboundXcmpMessages (max_values: None, max_size: None, mode: Measured)
	fn transfer_asset_via_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `542`
		//  Estimated: `17786`
		// Minimum execution time: 2_415_358 nanoseconds.
		Weight::from_parts(2_474_758_000, 17786)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: BridgeAssetsTransfer Bridges (r:1 w:1)
	/// Proof: BridgeAssetsTransfer Bridges (max_values: None, max_size: Some(1899), added: 4374, mode: MaxEncodedLen)
	fn add_bridge_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `4374`
		// Minimum execution time: 345_841 nanoseconds.
		Weight::from_parts(361_206_000, 4374)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeAssetsTransfer Bridges (r:1 w:1)
	/// Proof: BridgeAssetsTransfer Bridges (max_values: None, max_size: Some(1899), added: 4374, mode: MaxEncodedLen)
	fn remove_bridge_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `4374`
		// Minimum execution time: 367_373 nanoseconds.
		Weight::from_parts(379_332_000, 4374)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BridgeAssetsTransfer Bridges (r:1 w:1)
	/// Proof: BridgeAssetsTransfer Bridges (max_values: None, max_size: Some(1899), added: 4374, mode: MaxEncodedLen)
	fn update_bridge_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `4374`
		// Minimum execution time: 385_827 nanoseconds.
		Weight::from_parts(394_729_000, 4374)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
