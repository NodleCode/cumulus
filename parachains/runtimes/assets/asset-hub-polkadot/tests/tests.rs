// This file is part of Cumulus.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for the Statemint (Polkadot Assets Hub) chain.

use asset_hub_polkadot_runtime::xcm_config::{
	bridging, AssetFeeAsExistentialDepositMultiplierFeeCharger, CheckingAccount, DotLocation,
	ForeignCreatorsSovereignAccountOf, LocationToAccountId, TrustBackedAssetsPalletLocation,
	XcmConfig,
};
pub use asset_hub_polkadot_runtime::{
	constants::fee::WeightToFee, AssetDeposit, Assets, Balances, ExistentialDeposit, ForeignAssets,
	ForeignAssetsInstance, MetadataDepositBase, MetadataDepositPerByte, ParachainSystem, Runtime,
	RuntimeCall, RuntimeEvent, SessionKeys, System, TrustBackedAssetsInstance, XcmpQueue,
};
use asset_test_utils::{CollatorSessionKey, CollatorSessionKeys, ExtBuilder, RuntimeHelper};
use codec::{Decode, Encode};
use cumulus_primitives_utility::ChargeWeightInFungibles;
use frame_support::{
	assert_noop, assert_ok,
	traits::{fungibles::InspectEnumerable, Contains},
	weights::{Weight, WeightToFee as WeightToFeeT},
};
use parachains_common::{
	AccountId, AssetHubPolkadotAuraId as AuraId, AssetIdForTrustBackedAssets, Balance,
};
use sp_runtime::traits::MaybeEquivalence;
use xcm::latest::prelude::*;
use xcm_executor::traits::{Identity, JustTry, WeightTrader};

const ALICE: [u8; 32] = [1u8; 32];
const SOME_ASSET_ADMIN: [u8; 32] = [5u8; 32];

type AssetIdForTrustBackedAssetsConvert =
	assets_common::AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocation>;

fn collator_session_key(account: [u8; 32]) -> CollatorSessionKey<Runtime> {
	CollatorSessionKey::new(
		AccountId::from(account),
		AccountId::from(account),
		SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(account)) },
	)
}

fn collator_session_keys() -> CollatorSessionKeys<Runtime> {
	CollatorSessionKeys::default().add(collator_session_key(ALICE))
}

#[test]
fn test_asset_xcm_trader() {
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			// We need root origin to create a sufficient asset
			let minimum_asset_balance = 333333333_u128;
			let local_asset_id = 1;
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				local_asset_id.into(),
				AccountId::from(ALICE).into(),
				true,
				minimum_asset_balance
			));

			// We first mint enough asset for the account to exist for assets
			assert_ok!(Assets::mint(
				RuntimeHelper::<Runtime>::origin_of(AccountId::from(ALICE)),
				local_asset_id.into(),
				AccountId::from(ALICE).into(),
				minimum_asset_balance
			));

			// get asset id as multilocation
			let asset_multilocation =
				AssetIdForTrustBackedAssetsConvert::convert_back(&local_asset_id).unwrap();

			// Set Alice as block author, who will receive fees
			RuntimeHelper::<Runtime>::run_to_block(2, Some(AccountId::from(ALICE)));

			// We are going to buy 400e9 weight
			// Because of the ED being higher in kusama's asset hub
			// and not to complicate things, we use a little
			// bit more of weight
			let bought = Weight::from_parts(400_000_000_000u64, 0);

			// Lets calculate amount needed
			let asset_amount_needed =
				AssetFeeAsExistentialDepositMultiplierFeeCharger::charge_weight_in_fungibles(
					local_asset_id,
					bought,
				)
				.expect("failed to compute");

			// Lets pay with: asset_amount_needed + asset_amount_extra
			let asset_amount_extra = 100_u128;
			let asset: MultiAsset =
				(asset_multilocation, asset_amount_needed + asset_amount_extra).into();

			let mut trader = <XcmConfig as xcm_executor::Config>::Trader::new();

			// Lets buy_weight and make sure buy_weight does not return an error
			let unused_assets = trader.buy_weight(bought, asset.into()).expect("Expected Ok");
			// Check whether a correct amount of unused assets is returned
			assert_ok!(
				unused_assets.ensure_contains(&(asset_multilocation, asset_amount_extra).into())
			);

			// Drop trader
			drop(trader);

			// Make sure author(Alice) has received the amount
			assert_eq!(
				Assets::balance(local_asset_id, AccountId::from(ALICE)),
				minimum_asset_balance + asset_amount_needed
			);

			// We also need to ensure the total supply increased
			assert_eq!(
				Assets::total_supply(local_asset_id),
				minimum_asset_balance + asset_amount_needed
			);
		});
}

#[test]
fn test_asset_xcm_trader_with_refund() {
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			// We need root origin to create a sufficient asset
			// We set existential deposit to be identical to the one for Balances first
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				1.into(),
				AccountId::from(ALICE).into(),
				true,
				ExistentialDeposit::get()
			));

			// We first mint enough asset for the account to exist for assets
			assert_ok!(Assets::mint(
				RuntimeHelper::<Runtime>::origin_of(AccountId::from(ALICE)),
				1.into(),
				AccountId::from(ALICE).into(),
				ExistentialDeposit::get()
			));

			let mut trader = <XcmConfig as xcm_executor::Config>::Trader::new();

			// Set Alice as block author, who will receive fees
			RuntimeHelper::<Runtime>::run_to_block(2, Some(AccountId::from(ALICE)));

			// We are going to buy 400e9 weight
			// Because of the ED being higher in kusama's asset hub
			// and not to complicate things, we use a little
			// bit more of weight
			let bought = Weight::from_parts(400_000_000_000u64, 0);

			let asset_multilocation = AssetIdForTrustBackedAssetsConvert::convert_back(&1).unwrap();

			// lets calculate amount needed
			let amount_bought = WeightToFee::weight_to_fee(&bought);

			let asset: MultiAsset = (asset_multilocation, amount_bought).into();

			// Make sure buy_weight does not return an error
			assert_ok!(trader.buy_weight(bought, asset.clone().into()));

			// Make sure again buy_weight does return an error
			// This assert relies on the fact, that we use `TakeFirstAssetTrader` in `WeightTrader` tuple chain, which cannot be called twice
			assert_noop!(trader.buy_weight(bought, asset.into()), XcmError::TooExpensive);

			// We actually use half of the weight
			let weight_used = bought / 2;

			// Make sure refurnd works.
			let amount_refunded = WeightToFee::weight_to_fee(&(bought - weight_used));

			assert_eq!(
				trader.refund_weight(bought - weight_used),
				Some((asset_multilocation, amount_refunded).into())
			);

			// Drop trader
			drop(trader);

			// We only should have paid for half of the bought weight
			let fees_paid = WeightToFee::weight_to_fee(&weight_used);

			assert_eq!(
				Assets::balance(1, AccountId::from(ALICE)),
				ExistentialDeposit::get() + fees_paid
			);

			// We also need to ensure the total supply increased
			assert_eq!(Assets::total_supply(1), ExistentialDeposit::get() + fees_paid);
		});
}

#[test]
fn test_asset_xcm_trader_refund_not_possible_since_amount_less_than_ed() {
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			// We need root origin to create a sufficient asset
			// We set existential deposit to be identical to the one for Balances first
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				1.into(),
				AccountId::from(ALICE).into(),
				true,
				ExistentialDeposit::get()
			));

			let mut trader = <XcmConfig as xcm_executor::Config>::Trader::new();

			// Set Alice as block author, who will receive fees
			RuntimeHelper::<Runtime>::run_to_block(2, Some(AccountId::from(ALICE)));

			// We are going to buy 50e9 weight
			// Because of the ED being higher in kusama's asset hub
			// and not to complicate things, we use a little
			// bit more of weight
			let bought = Weight::from_parts(50_000_000_000u64, 0);

			let asset_multilocation = AssetIdForTrustBackedAssetsConvert::convert_back(&1).unwrap();

			let amount_bought = WeightToFee::weight_to_fee(&bought);

			assert!(
				amount_bought < ExistentialDeposit::get(),
				"we are testing what happens when the amount does not exceed ED"
			);

			let asset: MultiAsset = (asset_multilocation, amount_bought).into();

			// Buy weight should return an error
			assert_noop!(trader.buy_weight(bought, asset.into()), XcmError::TooExpensive);

			// not credited since the ED is higher than this value
			assert_eq!(Assets::balance(1, AccountId::from(ALICE)), 0);

			// We also need to ensure the total supply did not increase
			assert_eq!(Assets::total_supply(1), 0);
		});
}

#[test]
fn test_that_buying_ed_refund_does_not_refund() {
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			// We need root origin to create a sufficient asset
			// We set existential deposit to be identical to the one for Balances first
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				1.into(),
				AccountId::from(ALICE).into(),
				true,
				ExistentialDeposit::get()
			));

			let mut trader = <XcmConfig as xcm_executor::Config>::Trader::new();

			// Set Alice as block author, who will receive fees
			RuntimeHelper::<Runtime>::run_to_block(2, Some(AccountId::from(ALICE)));

			// We are gonna buy ED
			let bought = Weight::from_parts(ExistentialDeposit::get().try_into().unwrap(), 0);

			let asset_multilocation = AssetIdForTrustBackedAssetsConvert::convert_back(&1).unwrap();

			let amount_bought = WeightToFee::weight_to_fee(&bought);

			assert!(
				amount_bought < ExistentialDeposit::get(),
				"we are testing what happens when the amount does not exceed ED"
			);

			// We know we will have to buy at least ED, so lets make sure first it will
			// fail with a payment of less than ED
			let asset: MultiAsset = (asset_multilocation, amount_bought).into();
			assert_noop!(trader.buy_weight(bought, asset.into()), XcmError::TooExpensive);

			// Now lets buy ED at least
			let asset: MultiAsset = (asset_multilocation, ExistentialDeposit::get()).into();

			// Buy weight should work
			assert_ok!(trader.buy_weight(bought, asset.into()));

			// Should return None. We have a specific check making sure we dont go below ED for
			// drop payment
			assert_eq!(trader.refund_weight(bought), None);

			// Drop trader
			drop(trader);

			// Make sure author(Alice) has received the amount
			assert_eq!(Assets::balance(1, AccountId::from(ALICE)), ExistentialDeposit::get());

			// We also need to ensure the total supply increased
			assert_eq!(Assets::total_supply(1), ExistentialDeposit::get());
		});
}

#[test]
fn test_asset_xcm_trader_not_possible_for_non_sufficient_assets() {
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			// Create a non-sufficient asset
			let minimum_asset_balance = 1_000_000_u128;
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				1.into(),
				AccountId::from(ALICE).into(),
				false,
				minimum_asset_balance
			));

			// We first mint enough asset for the account to exist for assets
			assert_ok!(Assets::mint(
				RuntimeHelper::<Runtime>::origin_of(AccountId::from(ALICE)),
				1.into(),
				AccountId::from(ALICE).into(),
				minimum_asset_balance
			));

			let mut trader = <XcmConfig as xcm_executor::Config>::Trader::new();

			// Set Alice as block author, who will receive fees
			RuntimeHelper::<Runtime>::run_to_block(2, Some(AccountId::from(ALICE)));

			// We are going to buy 400e9 weight
			// Because of the ED being higher in kusama's asset hub
			// and not to complicate things, we use a little
			// bit more of weight
			let bought = Weight::from_parts(400_000_000_000u64, 0);

			// lets calculate amount needed
			let asset_amount_needed = WeightToFee::weight_to_fee(&bought);

			let asset_multilocation = AssetIdForTrustBackedAssetsConvert::convert_back(&1).unwrap();

			let asset: MultiAsset = (asset_multilocation, asset_amount_needed).into();

			// Make sure again buy_weight does return an error
			assert_noop!(trader.buy_weight(bought, asset.into()), XcmError::TooExpensive);

			// Drop trader
			drop(trader);

			// Make sure author(Alice) has NOT received the amount
			assert_eq!(Assets::balance(1, AccountId::from(ALICE)), minimum_asset_balance);

			// We also need to ensure the total supply NOT increased
			assert_eq!(Assets::total_supply(1), minimum_asset_balance);
		});
}

#[test]
fn test_assets_balances_api_works() {
	use assets_common::runtime_api::runtime_decl_for_fungibles_api::FungiblesApi;

	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			let local_asset_id = 1;
			let foreign_asset_id_multilocation =
				MultiLocation { parents: 1, interior: X2(Parachain(1234), GeneralIndex(12345)) };

			// check before
			assert_eq!(Assets::balance(local_asset_id, AccountId::from(ALICE)), 0);
			assert_eq!(
				ForeignAssets::balance(foreign_asset_id_multilocation, AccountId::from(ALICE)),
				0
			);
			assert_eq!(Balances::free_balance(AccountId::from(ALICE)), 0);
			assert!(Runtime::query_account_balances(AccountId::from(ALICE))
				.unwrap()
				.try_as::<MultiAssets>()
				.unwrap()
				.is_none());

			// Drip some balance
			use frame_support::traits::fungible::Mutate;
			let some_currency = ExistentialDeposit::get();
			Balances::mint_into(&AccountId::from(ALICE), some_currency).unwrap();

			// We need root origin to create a sufficient asset
			let minimum_asset_balance = 3333333_u128;
			assert_ok!(Assets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				local_asset_id.into(),
				AccountId::from(ALICE).into(),
				true,
				minimum_asset_balance
			));

			// We first mint enough asset for the account to exist for assets
			assert_ok!(Assets::mint(
				RuntimeHelper::<Runtime>::origin_of(AccountId::from(ALICE)),
				local_asset_id.into(),
				AccountId::from(ALICE).into(),
				minimum_asset_balance
			));

			// create foreign asset
			let foreign_asset_minimum_asset_balance = 3333333_u128;
			assert_ok!(ForeignAssets::force_create(
				RuntimeHelper::<Runtime>::root_origin(),
				foreign_asset_id_multilocation,
				AccountId::from(SOME_ASSET_ADMIN).into(),
				false,
				foreign_asset_minimum_asset_balance
			));

			// We first mint enough asset for the account to exist for assets
			assert_ok!(ForeignAssets::mint(
				RuntimeHelper::<Runtime>::origin_of(AccountId::from(SOME_ASSET_ADMIN)),
				foreign_asset_id_multilocation,
				AccountId::from(ALICE).into(),
				6 * foreign_asset_minimum_asset_balance
			));

			// check after
			assert_eq!(
				Assets::balance(local_asset_id, AccountId::from(ALICE)),
				minimum_asset_balance
			);
			assert_eq!(
				ForeignAssets::balance(foreign_asset_id_multilocation, AccountId::from(ALICE)),
				6 * minimum_asset_balance
			);
			assert_eq!(Balances::free_balance(AccountId::from(ALICE)), some_currency);

			let result: MultiAssets = Runtime::query_account_balances(AccountId::from(ALICE))
				.unwrap()
				.try_into()
				.unwrap();
			assert_eq!(result.len(), 3);

			// check currency
			assert!(result.inner().iter().any(|asset| asset.eq(
				&assets_common::fungible_conversion::convert_balance::<DotLocation, Balance>(
					some_currency
				)
				.unwrap()
			)));
			// check trusted asset
			assert!(result.inner().iter().any(|asset| asset.eq(&(
				AssetIdForTrustBackedAssetsConvert::convert_back(&local_asset_id).unwrap(),
				minimum_asset_balance
			)
				.into())));
			// check foreign asset
			assert!(result.inner().iter().any(|asset| asset.eq(&(
				Identity::convert_back(&foreign_asset_id_multilocation).unwrap(),
				6 * foreign_asset_minimum_asset_balance
			)
				.into())));
		});
}

asset_test_utils::include_teleports_for_native_asset_works!(
	Runtime,
	XcmConfig,
	CheckingAccount,
	WeightToFee,
	ParachainSystem,
	collator_session_keys(),
	ExistentialDeposit::get(),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::PolkadotXcm(event)) => Some(event),
			_ => None,
		}
	}),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::XcmpQueue(event)) => Some(event),
			_ => None,
		}
	}),
	1000
);

asset_test_utils::include_teleports_for_foreign_assets_works!(
	Runtime,
	XcmConfig,
	CheckingAccount,
	WeightToFee,
	ParachainSystem,
	ForeignCreatorsSovereignAccountOf,
	ForeignAssetsInstance,
	asset_test_utils::CollatorSessionKeys::new(
		AccountId::from(ALICE),
		AccountId::from(ALICE),
		SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) }
	),
	ExistentialDeposit::get(),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::PolkadotXcm(event)) => Some(event),
			_ => None,
		}
	}),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::XcmpQueue(event)) => Some(event),
			_ => None,
		}
	})
);

asset_test_utils::include_asset_transactor_transfer_with_local_consensus_currency_works!(
	Runtime,
	XcmConfig,
	collator_session_keys(),
	ExistentialDeposit::get(),
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
		assert!(ForeignAssets::asset_ids().collect::<Vec<_>>().is_empty());
	}),
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
		assert!(ForeignAssets::asset_ids().collect::<Vec<_>>().is_empty());
	})
);

asset_test_utils::include_asset_transactor_transfer_with_pallet_assets_instance_works!(
	asset_transactor_transfer_with_trust_backed_assets_works,
	Runtime,
	XcmConfig,
	TrustBackedAssetsInstance,
	AssetIdForTrustBackedAssets,
	AssetIdForTrustBackedAssetsConvert,
	collator_session_keys(),
	ExistentialDeposit::get(),
	12345,
	Box::new(|| {
		assert!(ForeignAssets::asset_ids().collect::<Vec<_>>().is_empty());
	}),
	Box::new(|| {
		assert!(ForeignAssets::asset_ids().collect::<Vec<_>>().is_empty());
	})
);

asset_test_utils::include_asset_transactor_transfer_with_pallet_assets_instance_works!(
	asset_transactor_transfer_with_foreign_assets_works,
	Runtime,
	XcmConfig,
	ForeignAssetsInstance,
	MultiLocation,
	JustTry,
	asset_test_utils::CollatorSessionKeys::new(
		AccountId::from(ALICE),
		AccountId::from(ALICE),
		SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) }
	),
	ExistentialDeposit::get(),
	MultiLocation { parents: 1, interior: X2(Parachain(1313), GeneralIndex(12345)) },
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
	}),
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
	})
);

asset_test_utils::include_create_and_manage_foreign_assets_for_local_consensus_parachain_assets_works!(
	Runtime,
	XcmConfig,
	WeightToFee,
	ForeignCreatorsSovereignAccountOf,
	ForeignAssetsInstance,
	MultiLocation,
	JustTry,
	asset_test_utils::CollatorSessionKeys::new(
		AccountId::from(ALICE),
		AccountId::from(ALICE),
		SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) }
	),
	ExistentialDeposit::get(),
	AssetDeposit::get(),
	MetadataDepositBase::get(),
	MetadataDepositPerByte::get(),
	Box::new(|pallet_asset_call| RuntimeCall::ForeignAssets(pallet_asset_call).encode()),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::ForeignAssets(pallet_asset_event)) => Some(pallet_asset_event),
			_ => None,
		}
	}),
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
		assert!(ForeignAssets::asset_ids().collect::<Vec<_>>().is_empty());
	}),
	Box::new(|| {
		assert!(Assets::asset_ids().collect::<Vec<_>>().is_empty());
		assert_eq!(ForeignAssets::asset_ids().collect::<Vec<_>>().len(), 1);
	})
);

fn bridging_to_asset_hub_kusama() -> asset_test_utils::test_cases_over_bridge::TestBridgingConfig {
	asset_test_utils::test_cases_over_bridge::TestBridgingConfig {
		bridged_network: bridging::KusamaNetwork::get(),
		local_bridge_hub_para_id: bridging::BridgeHubPolkadotParaId::get(),
		local_bridge_hub_location: bridging::BridgeHubPolkadot::get(),
		bridged_target_location: bridging::AssetHubKusama::get(),
	}
}

#[test]
fn limited_reserve_transfer_assets_for_native_asset_over_bridge_works() {
	asset_test_utils::test_cases_over_bridge::limited_reserve_transfer_assets_for_native_asset_works::<
		Runtime,
		XcmConfig,
		ParachainSystem,
		XcmpQueue,
		LocationToAccountId,
	>(
		collator_session_keys(),
		ExistentialDeposit::get(),
		AccountId::from(ALICE),
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::PolkadotXcm(event)) => Some(event),
				_ => None,
			}
		}),
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::XcmpQueue(event)) => Some(event),
				_ => None,
			}
		}),
		bridging_to_asset_hub_kusama,
		WeightLimit::Unlimited,
	)
}

#[test]
fn receive_reserve_asset_deposited_ksm_from_asset_hub_kusama_works() {
	const BLOCK_AUTHOR_ACCOUNT: [u8; 32] = [13; 32];
	asset_test_utils::test_cases_over_bridge::receive_reserve_asset_deposited_from_different_consensus_works::<
		Runtime,
		XcmConfig,
		LocationToAccountId,
		ForeignAssetsInstance,
	>(
		collator_session_keys().add(collator_session_key(BLOCK_AUTHOR_ACCOUNT)),
		ExistentialDeposit::get(),
		AccountId::from([73; 32]),
		AccountId::from(BLOCK_AUTHOR_ACCOUNT),
		// receiving DOTs
		(MultiLocation { parents: 2, interior: X1(GlobalConsensus(Kusama)) }, 1000000000000, 1_000_000_000),
		bridging_to_asset_hub_kusama,
		(X1(PalletInstance(53)), GlobalConsensus(Kusama), X1(Parachain(1000)))
	)
}

/// Tests expected configuration of isolated `pallet_xcm::Config::XcmReserveTransferFilter`.
#[test]
fn xcm_reserve_transfer_filter_works() {
	// prepare assets
	let only_native_assets = || vec![MultiAsset::from((DotLocation::get(), 1000))];
	let only_trust_backed_assets = || {
		vec![MultiAsset::from((
			AssetIdForTrustBackedAssetsConvert::convert_back(&12345).unwrap(),
			2000,
		))]
	};
	let only_sibling_foreign_assets =
		|| vec![MultiAsset::from((MultiLocation::new(1, X1(Parachain(12345))), 3000))];
	let only_different_global_consensus_foreign_assets = || {
		vec![MultiAsset::from((
			MultiLocation::new(2, X2(GlobalConsensus(Kusama), Parachain(12345))),
			4000,
		))]
	};

	// prepare destinations
	let relaychain = MultiLocation::parent();
	let sibling_parachain = MultiLocation::new(1, X1(Parachain(1000)));
	let different_global_consensus_parachain_other_then_asset_hub_kusama =
		MultiLocation::new(2, X2(GlobalConsensus(Kusama), Parachain(12345)));
	let bridged_asset_hub = bridging::AssetHubKusama::get();

	// prepare expected test data sets: (destination, assets, expected_result)
	let test_data = vec![
		(relaychain, only_native_assets(), true),
		(relaychain, only_trust_backed_assets(), true),
		(relaychain, only_sibling_foreign_assets(), true),
		(relaychain, only_different_global_consensus_foreign_assets(), true),
		(sibling_parachain, only_native_assets(), true),
		(sibling_parachain, only_trust_backed_assets(), true),
		(sibling_parachain, only_sibling_foreign_assets(), true),
		(sibling_parachain, only_different_global_consensus_foreign_assets(), true),
		(
			different_global_consensus_parachain_other_then_asset_hub_kusama,
			only_native_assets(),
			true,
		),
		(
			different_global_consensus_parachain_other_then_asset_hub_kusama,
			only_trust_backed_assets(),
			true,
		),
		(
			different_global_consensus_parachain_other_then_asset_hub_kusama,
			only_sibling_foreign_assets(),
			true,
		),
		(
			different_global_consensus_parachain_other_then_asset_hub_kusama,
			only_different_global_consensus_foreign_assets(),
			true,
		),
		(bridged_asset_hub, only_native_assets(), true),
		(bridged_asset_hub, only_trust_backed_assets(), false),
		(bridged_asset_hub, only_sibling_foreign_assets(), false),
		(bridged_asset_hub, only_different_global_consensus_foreign_assets(), false),
	];

	// lets test filter with test data
	type XcmReserveTransferFilter = <Runtime as pallet_xcm::Config>::XcmReserveTransferFilter;
	for (dest, assets, expected_result) in test_data {
		assert_eq!(expected_result, XcmReserveTransferFilter::contains(&(dest, assets)));
	}
}
