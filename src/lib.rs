mod abi;
mod db;
mod pb;
use pb::erc20::{
    Delegator, Events, Indexer, StakeDelegated, StakeDelegatedEvents, StakeDeposited,
    StakeDepositedEvents, StakeWithdrawn, StakeWithdrawnEvents, StakeDelegatedLocked, StakeDelegatedLockedEvents, Transfer, Transfers,
};
use std::str::FromStr;
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store;
use substreams::store::{DeltaBigInt, DeltaString, Deltas};
use substreams::{
    hex, log, store::StoreAddBigInt, store::StoreSetIfNotExists, store::StoreSetIfNotExistsString,
    store::StoreSetProto, Hex,
};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::Event;
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

// Contract Addresses
const GRAPH_TOKEN_ADDRESS: [u8; 20] = hex!("c944E90C64B2c07662A292be6244BDf05Cda44a7");
const STAKING_CONTRACT: [u8; 20] = hex!("F55041E37E12cD407ad00CE2910B8269B01263b9");

substreams_ethereum::init!();

// -------------------- INITIAL MAPS --------------------

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut transfers = vec![];
    let mut stake_deposited_events = vec![];
    let mut stake_withdrawn_events = vec![];
    let mut stake_delegated_events = vec![];
    let mut stake_delegated_locked_events = vec![];

    for log in blk.logs() {
        if !(&Hex(&GRAPH_TOKEN_ADDRESS).to_string() == &Hex(&log.address()).to_string()
            || &Hex(&STAKING_CONTRACT).to_string() == &Hex(&log.address()).to_string())
        {
            continue;
        }

        if let Some(event) = abi::erc20::events::Transfer::match_and_decode(log) {
            transfers.push(Transfer {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each Transfer needs a unique id
                from: event.from,
                to: event.to,
                value: event.value.to_string(), // Value is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.block_index() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeDeposited::match_and_decode(log) {
            stake_deposited_events.push(StakeDeposited {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.block_index() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeWithdrawn::match_and_decode(log) {
            stake_withdrawn_events.push(StakeWithdrawn {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.block_index() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeDelegated::match_and_decode(log) {
            stake_delegated_events.push(StakeDelegated {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                delegator: event.delegator,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.block_index() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeDelegatedLocked::match_and_decode(log) {
            stake_delegated_locked_events.push(StakeDelegatedLocked {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                delegator: event.delegator,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.block_index() as u64,
            });
        }
    }

    events.transfers = Some(Transfers {
        transfers: transfers,
    });
    events.stake_deposited_events = Some(StakeDepositedEvents {
        stake_deposited_events: stake_deposited_events,
    });
    events.stake_withdrawn_events = Some(StakeWithdrawnEvents {
        stake_withdrawn_events: stake_withdrawn_events,
    });
    events.stake_delegated_events = Some(StakeDelegatedEvents {
        stake_delegated_events: stake_delegated_events,
    });
    events.stake_delegated_locked_events = Some(StakeDelegatedLockedEvents {
        stake_delegated_locked_events: stake_delegated_locked_events,
    });

    Ok(events)
}

// -------------------- STORES --------------------
#[substreams::handlers::store]
fn store_grt_balances(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        s.add(
            transfer.ordinal,
            generate_key(&transfer.from),
            BigInt::from_str(&transfer.value).unwrap().neg(),
        );
        s.add(
            transfer.ordinal,
            generate_key(&transfer.to),
            BigInt::from_str(&transfer.value).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_grt_global(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        if transfer.to == NULL_ADDRESS {
            s.add(
                transfer.ordinal,
                "totalSupply",
                BigInt::from_str(&transfer.value).unwrap().neg(),
            );
            s.add(
                transfer.ordinal,
                "totalGRTBurned",
                BigInt::from_str(&transfer.value).unwrap(),
            );
        }
        if transfer.from == NULL_ADDRESS {
            s.add(
                transfer.ordinal,
                "totalSupply",
                BigInt::from_str(&transfer.value).unwrap(),
            );
            s.add(
                transfer.ordinal,
                "totalGRTMinted",
                BigInt::from_str(&transfer.value).unwrap(),
            );
        }
    }
}

#[substreams::handlers::store]
fn store_indexer_stakes(events: Events, s: StoreAddBigInt) {
    let stake_deposited_events = events.stake_deposited_events.unwrap();
    let stake_withdrawn_events = events.stake_withdrawn_events.unwrap();

    for stakeDeposited in stake_deposited_events.stake_deposited_events {
        s.add(
            stakeDeposited.ordinal,
            generate_key(&stakeDeposited.indexer),
            BigInt::from_str(&stakeDeposited.tokens).unwrap(),
        );
        s.add(
            stakeDeposited.ordinal,
            "totalTokensStaked",
            BigInt::from_str(&stakeDeposited.tokens).unwrap(),
        );
    }

    for stakeWithdrawn in stake_withdrawn_events.stake_withdrawn_events {
        s.add(
            stakeWithdrawn.ordinal,
            generate_key(&stakeWithdrawn.indexer),
            BigInt::from_str(&stakeWithdrawn.tokens).unwrap().neg(),
        );
        s.add(
            stakeWithdrawn.ordinal,
            "totalTokensStaked",
            BigInt::from_str(&stakeWithdrawn.tokens).unwrap().neg(),
        );
    }
}

// DelegatedStake and Delegator entities track the cumulative delegated stake, not the total amount
#[substreams::handlers::store]
fn store_cumulative_delegated_stakes(events: Events, s: StoreAddBigInt) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();

    for stakeDelegated in stake_delegated_events.stake_delegated_events {
        s.add(
            stakeDelegated.ordinal,
            generate_key_delegated_stake(&stakeDelegated.delegator, &stakeDelegated.indexer),
            BigInt::from_str(&stakeDelegated.tokens).unwrap(),
        );
    }
}

// Indexer and GraphNetwork entities track the total delegated stake, not the cumulative amount
#[substreams::handlers::store]
fn store_total_delegated_stakes(events: Events, s: StoreAddBigInt) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();
    let stake_delegated_locked_events = events.stake_delegated_locked_events.unwrap();

    for stakeDelegated in stake_delegated_events.stake_delegated_events {
        s.add(
            stakeDelegated.ordinal,
            generate_key(&stakeDelegated.indexer),
            BigInt::from_str(&stakeDelegated.tokens).unwrap(),
        );
        s.add(
            stakeDelegated.ordinal,
            "totalTokensDelegated",
            BigInt::from_str(&stakeDelegated.tokens).unwrap(),
        );
    }

    for stakeDelegatedLocked in stake_delegated_locked_events.stake_delegated_locked_events {
        s.add(
            stakeDelegatedLocked.ordinal,
            generate_key(&stakeDelegatedLocked.indexer),
            BigInt::from_str(&stakeDelegatedLocked.tokens).unwrap().neg(),
        );
        s.add(
            stakeDelegatedLocked.ordinal,
            "totalTokensDelegated",
            BigInt::from_str(&stakeDelegatedLocked.tokens).unwrap().neg(),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_indexer(events: Events, s: StoreSetIfNotExistsString) {
    let stake_deposited_events = events.stake_deposited_events.unwrap();
    for stakeDeposited in stake_deposited_events.stake_deposited_events {
        s.set_if_not_exists(
            stakeDeposited.ordinal,
            generate_key(&stakeDeposited.indexer),
                &generate_key(&stakeDeposited.indexer),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_delegator(events: Events, s: StoreSetIfNotExistsString) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();
    for stakeDelegated in stake_delegated_events.stake_delegated_events {
        s.set_if_not_exists(
            stakeDelegated.ordinal,
            generate_key_delegated_stake(&stakeDelegated.delegator, &stakeDelegated.indexer),
            &generate_key(&stakeDelegated.delegator)
        );
    }
}

// -------------------- MAPS FOR ENTITY CHANGES --------------------
// We have an entity change map for each entity in our subgraph schema.graphql
// These maps take necessary stores or maps as inputs and create/update corresponding entitites in the subgraph using entity changes

#[substreams::handlers::map]
pub fn map_graph_network_entities(
    grt_global_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::grt_global_change(grt_global_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_graph_account_entities(
    grt_balance_deltas: Deltas<DeltaBigInt>,
    graph_account_indexer_deltas: Deltas<DeltaString>,
    graph_account_delegator_deltas: Deltas<DeltaString>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::grt_balance_change(grt_balance_deltas, &mut entity_changes);
    db::graph_account_indexer_change(graph_account_indexer_deltas, &mut entity_changes);
    db::graph_account_delegator_change(graph_account_delegator_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_indexer_entities(
    indexer_stake_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::indexer_stake_change(indexer_stake_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_delegated_stake_entities(
    cumulative_delegated_stake_deltas: Deltas<DeltaBigInt>,
    total_delegated_stake_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::delegated_stake_change(cumulative_delegated_stake_deltas, total_delegated_stake_deltas, &mut entity_changes);
    Ok(entity_changes)
}

// -------------------- GRAPH_OUT --------------------
// Final map for executing all entity change maps together
// Run this map to check the health of the entire substream

#[substreams::handlers::map]
pub fn graph_out(
    graph_network_entities: EntityChanges,
    graph_account_entities: EntityChanges,
    indexer_entities: EntityChanges,
    delegated_stake_entities: EntityChanges,
) -> Result<EntityChanges, substreams::errors::Error> {
    Ok(EntityChanges {
        entity_changes: [
            graph_network_entities.entity_changes,
            graph_account_entities.entity_changes,
            indexer_entities.entity_changes,
            delegated_stake_entities.entity_changes,
        ]
        .concat(),
    })
}

// -------------------- KEY GENERATORS --------------------
fn generate_key(account: &Vec<u8>) -> String {
    return Hex(account).to_string();
}

fn generate_key_delegated_stake(delegator: &Vec<u8>, indexer: &Vec<u8>) -> String {
    return format!(
        "{}:{}",
        Hex(delegator).to_string(),
        Hex(indexer).to_string()
    );
}
