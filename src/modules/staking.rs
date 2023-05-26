use crate::pb::erc20::*;
use crate::utils;
use std::ops::Sub;
use std::str::FromStr;
use substreams::prelude::*;
use substreams::{hex, log, Hex};
use substreams::scalar::BigInt;
use substreams::{
    store::StoreAddBigInt, store::StoreSetIfNotExists, store::StoreSetIfNotExistsString,
    store::StoreSetProto,
};
use substreams::errors::Error;

// DelegatedStake and Delegator entities track the cumulative delegated stake, not the total amount
#[substreams::handlers::store]
fn store_staked_tokens(storage_changes: StorageChanges, s: StoreAddBigInt) {
    let indexer_stakes = storage_changes.indexer_stakes.unwrap();

    for indexer_stake in indexer_stakes.indexer_stakes {
        s.add(
            indexer_stake.ordinal,
            "totalTokensStaked",
            BigInt::from_str(&indexer_stake.new_stake)
                .unwrap()
                .sub(BigInt::from_str(&indexer_stake.old_stake).unwrap()),
        );
    }
}

// DelegatedStake and Delegator entities track the cumulative delegated stake, not the total amount
#[substreams::handlers::store]
fn store_cumulative_delegated_stakes(events: Events, s: StoreAddBigInt) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();

    for stake_delegated in stake_delegated_events.stake_delegated_events {
        s.add(
            stake_delegated.ordinal,
            utils::generate_key_delegated_stake(
                &stake_delegated.delegator,
                &stake_delegated.indexer,
            ),
            BigInt::from_str(&stake_delegated.tokens).unwrap(),
        );
    }
}

// DelegatedStake and Delegator entities track the cumulative delegated stake, not the total amount
#[substreams::handlers::store]
fn store_cumulative_delegator_stakes(events: Events, s: StoreAddBigInt) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();

    for stake_delegated in stake_delegated_events.stake_delegated_events {
        s.add(
            stake_delegated.ordinal,
            utils::generate_key(&stake_delegated.delegator),
            BigInt::from_str(&stake_delegated.tokens).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_indexer(storage_changes: StorageChanges, s: StoreSetIfNotExistsString) {
    let indexer_stakes = storage_changes.indexer_stakes.unwrap();
    for indexer_stake in indexer_stakes.indexer_stakes {
        s.set_if_not_exists(
            indexer_stake.ordinal,
            utils::generate_key(&indexer_stake.indexer),
            &utils::generate_key(&indexer_stake.indexer),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_delegator(events: Events, s: StoreSetIfNotExistsString) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();
    for stake_delegated in stake_delegated_events.stake_delegated_events {
        s.set_if_not_exists(
            stake_delegated.ordinal,
            utils::generate_key_delegated_stake(
                &stake_delegated.delegator,
                &stake_delegated.indexer,
            ),
            &utils::generate_key(&stake_delegated.delegator),
        );
    }
}

#[substreams::handlers::store]
fn store_delegation_parameters(events: Events, s: StoreSetProto<DelegationParametersUpdated>) {
    let delegation_parameters_updated_events = events.delegation_parameters_updated_events.unwrap();
    for delegation_parameters_updated in
        delegation_parameters_updated_events.delegation_parameters_updated_events
    {
        s.set(
            delegation_parameters_updated.ordinal,
            utils::generate_key(&delegation_parameters_updated.indexer),
            &delegation_parameters_updated,
        );
    }
}

// Indexer and GraphNetwork entities track the total delegated stake, not the cumulative amount
#[substreams::handlers::store]
fn store_total_delegated_stakes(storage_changes: StorageChanges, s: StoreAddBigInt) {
    let delegation_pools = storage_changes.delegation_pools.unwrap();

    for delegation_pool in delegation_pools.delegation_pools {
        s.add(
            delegation_pool.ordinal,
            "totalTokensDelegated",
            BigInt::from_str(&delegation_pool.new_stake)
                .unwrap()
                .sub(BigInt::from_str(&delegation_pool.old_stake).unwrap()),
        );
    }
}


