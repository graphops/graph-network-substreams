mod abi;
mod pb;
mod db;
use pb::erc20::{ Events, Transfers, Transfer, StakeDeposited, StakeDepositedEvents };
use substreams::prelude::*;
use std::str::FromStr;
use substreams::{ log, hex, store::StoreAddBigInt, store::StoreSetProto, Hex };
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::store;
use substreams::errors::Error;
use substreams::store::{ Deltas, DeltaBigInt };
use substreams_ethereum::{ pb::eth::v2 as eth, NULL_ADDRESS };
use substreams::scalar::BigInt;
use substreams_ethereum::Event;

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

    for log in blk.logs() {
        if !(&Hex(&GRAPH_TOKEN_ADDRESS).to_string() == &Hex(&log.address()).to_string() || &Hex(&STAKING_CONTRACT).to_string() == &Hex(&log.address()).to_string() ) {
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
        }

        else if let Some(event) = abi::staking::events::StakeDeposited::match_and_decode(log) {
            stake_deposited_events.push(StakeDeposited {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
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

    Ok(events)
}

// -------------------- STORES --------------------
#[substreams::handlers::store]
fn store_grt_balances(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        s.add(
            transfer.ordinal,
            generate_key_transfer(&transfer.from),
            BigInt::from_str(&transfer.value).unwrap().neg()
        );
        s.add(
            transfer.ordinal,
            generate_key_transfer(&transfer.to),
            BigInt::from_str(&transfer.value).unwrap()
        );
    }
}

#[substreams::handlers::store]
fn store_grt_global(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        if transfer.to == NULL_ADDRESS {
            s.add(transfer.ordinal, "totalSupply", BigInt::from_str(&transfer.value).unwrap().neg());
            s.add(transfer.ordinal, "totalGRTBurned", BigInt::from_str(&transfer.value).unwrap());
        }
        if transfer.from == NULL_ADDRESS {
            s.add(
                transfer.ordinal,
                "totalSupply",
                BigInt::from_str(&transfer.value).unwrap()
            );
            s.add(transfer.ordinal, "totalGRTMinted", BigInt::from_str(&transfer.value).unwrap());
        }
    }
}

#[substreams::handlers::store]
fn store_indexer_stakes(events: Events, s: StoreAddBigInt) {
    let stake_deposited_events = events.stake_deposited_events.unwrap();
    for stakeDeposited in stake_deposited_events.stake_deposited_events {
        s.add(
            stakeDeposited.ordinal,
            generate_key_transfer(&stakeDeposited.indexer),
            BigInt::from_str(&stakeDeposited.tokens).unwrap()
        );
        s.add(
            stakeDeposited.ordinal,
            "totalTokensStaked",
            BigInt::from_str(&stakeDeposited.tokens).unwrap()
        );
    }
}

// -------------------- MAPS FOR ENTITY CHANGES --------------------
// We have an entity change map for each entity in our subgraph schema.graphql
// These maps take necessary stores or maps as inputs and create/update corresponding entitites in the subgraph using entity changes

#[substreams::handlers::map]
pub fn map_graph_network_entities(
    grt_global_deltas: Deltas<DeltaBigInt>
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::grt_global_change(grt_global_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_graph_account_entities(
    grt_balance_deltas: Deltas<DeltaBigInt>
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::grt_balance_change(grt_balance_deltas, &mut entity_changes);
    Ok(entity_changes)
}

// -------------------- GRAPH_OUT --------------------
// Final map for executing all entity change maps together
// Run this map to check the health of the entire substream

#[substreams::handlers::map]
pub fn graph_out(graph_network_entities: EntityChanges,
    graph_account_entities: EntityChanges,
) -> Result<EntityChanges, substreams::errors::Error> {
    Ok(EntityChanges {
        entity_changes: [
            graph_network_entities.entity_changes,
            graph_account_entities.entity_changes,
        ]
        .concat(),
    })
}

// -------------------- KEY GENERATORS --------------------
fn generate_key_transfer(holder: &Vec<u8>) -> String {
    return Hex(holder).to_string();
}