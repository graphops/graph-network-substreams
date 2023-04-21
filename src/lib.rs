mod abi;
mod db;
mod pb;
use pb::erc20::{
    Burned, BurnedEvents,CurationPool, CurationPools, DelegationParametersUpdated, DelegationParametersUpdatedEvents,
    DelegationPool, DelegationPools, Events, IndexerStake, IndexerStakes, RebateClaimed,
    RebateClaimedEvents, RewardsAssigned, RewardsAssignedEvents, Signalled, SignalledEvents,
    StakeDelegated, StakeDelegatedEvents, StakeDelegatedLocked, StakeDelegatedLockedEvents,
    StakeDeposited, StakeDepositedEvents, StakeWithdrawn, StakeWithdrawnEvents, StorageChanges,
    Transfer, Transfers,
};
use std::ops::Sub;
use std::str::FromStr;
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, DeltaString, Deltas};
use substreams::{
    hex, log, store::StoreAddBigInt, store::StoreSetIfNotExists, store::StoreSetIfNotExistsString,
    store::StoreSetProto, Hex,
};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::Event;
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};
use tiny_keccak::{Hasher, Keccak};

// Contract Addresses
const GRAPH_TOKEN_ADDRESS: [u8; 20] = hex!("c944E90C64B2c07662A292be6244BDf05Cda44a7");
const STAKING_CONTRACT: [u8; 20] = hex!("F55041E37E12cD407ad00CE2910B8269B01263b9");
const REWARDS_MANAGER_CONTRACT: [u8; 20] = hex!("9Ac758AB77733b4150A901ebd659cbF8cB93ED66");
const GNS_CONTRACT: [u8; 20] = hex!("aDcA0dd4729c8BA3aCf3E99F3A9f471EF37b6825");
const CURATION_CONTRACT: [u8; 20] = hex!("8FE00a685Bcb3B2cc296ff6FfEaB10acA4CE1538");

// Probably improve this to be a list/vec in the future, as this doesn't really scale later

substreams_ethereum::init!();

// -------------------- INITIAL MAPS --------------------
fn find_key<T: AsRef<[u8]>>(address: T, slot: u64, order: u32) -> [u8; 32] {
    // Pad the address with leading zeros to make it 32 bytes
    let padded_address = add_padding(address);

    // Convert the number to a byte array
    let padded_slot = add_padding(slot.to_be_bytes());

    // Concatenate the padded address and padded number
    let mut concat = Vec::new();
    concat.extend_from_slice(&padded_address);
    concat.extend_from_slice(&padded_slot);
    let keccak = keccak256(&concat);
    let key = increment_key(keccak, order);
    key
}

fn increment_key(mut keccak: [u8; 32], order: u32) -> [u8; 32] {
    let mut carry = order;
    for i in (0..32).rev() {
        let (result, new_carry) = keccak[i].overflowing_add(carry as u8);
        keccak[i] = result;
        carry = new_carry as u32;
        if carry == 0 {
            break;
        }
    }
    keccak
}

pub fn keccak256(data: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut out: [u8; 32] = [0; 32];
    hasher.update(data);
    hasher.finalize(&mut out);
    out
}

fn add_padding<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut padded_input = [0; 32];
    let input_bytes = input.as_ref();

    let input_len = input_bytes.len();
    let pad_len = padded_input.len() - input_len;

    padded_input[pad_len..].copy_from_slice(input_bytes);
    padded_input
}

#[substreams::handlers::map]
fn map_storage_changes(blk: eth::Block) -> Result<StorageChanges, Error> {
    let mut storage_changes = StorageChanges::default();
    let mut indexer_stakes = vec![];
    let mut delegation_pools = vec![];
    let mut curation_pools = vec![];

    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                if let Some(event) = abi::staking::events::StakeDeposited::match_and_decode(&log) {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("{:?}", &keccak_preimage);
                    }
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 14, 0) {
                                indexer_stakes.push(IndexerStake {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) = abi::staking::events::StakeWithdrawn::match_and_decode(&log) {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("{:?}", &keccak_preimage);
                    }
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 14, 0) {
                                indexer_stakes.push(IndexerStake {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) = abi::staking::events::StakeDelegated::match_and_decode(&log) {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 20, 2) {
                                delegation_pools.push(DelegationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) =
                    abi::staking::events::StakeDelegatedLocked::match_and_decode(&log)
                {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 20, 2) {
                                delegation_pools.push(DelegationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) = abi::staking::events::RebateClaimed::match_and_decode(&log) {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 20, 2) {
                                delegation_pools.push(DelegationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) =
                    abi::rewards_manager::events::RewardsAssigned::match_and_decode(&log)
                {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&STAKING_CONTRACT) {
                            if storage_change.key == find_key(&event.indexer, 20, 2) {
                                delegation_pools.push(DelegationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    indexer: event.indexer.clone(),
                                    new_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_stake: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) =
                    abi::curation::events::Signalled::match_and_decode(&log)
                {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&CURATION_CONTRACT) {
                            if storage_change.key == find_key(&event.subgraph_deployment_id, 15, 0) {
                                curation_pools.push(CurationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    subgraph_deployment_id: Hex(&event.subgraph_deployment_id).to_string(),
                                    new_signal: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_signal: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
                if let Some(event) =
                    abi::curation::events::Burned::match_and_decode(&log)
                {
                    for keccak_preimage in &call.keccak_preimages {
                        log::info!("keccakdelegated{:?}", &keccak_preimage);
                    }
                    log::info!("transaction: {} ", Hex(&trx.hash));
                    for storage_change in &call.storage_changes {
                        if storage_change.address.eq(&CURATION_CONTRACT) {
                            if storage_change.key == find_key(&event.subgraph_deployment_id, 15, 0) {
                                curation_pools.push(CurationPool {
                                    id: Hex(&trx.hash).to_string(),
                                    subgraph_deployment_id: Hex(&event.subgraph_deployment_id).to_string(),
                                    new_signal: BigInt::from_unsigned_bytes_be(
                                        &storage_change.new_value,
                                    )
                                    .into(),
                                    old_signal: BigInt::from_unsigned_bytes_be(
                                        &storage_change.old_value,
                                    )
                                    .into(),
                                    ordinal: log.ordinal,
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    //indexer_stakes.sort_by(|x, y| x.ordinal.cmp(&y.ordinal));
    storage_changes.indexer_stakes = Some(IndexerStakes {
        indexer_stakes: indexer_stakes,
    });
    storage_changes.delegation_pools = Some(DelegationPools {
        delegation_pools: delegation_pools,
    });
    storage_changes.curation_pools = Some(CurationPools {
        curation_pools: curation_pools,
    });

    Ok(storage_changes)
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut transfers = vec![];
    let mut stake_deposited_events = vec![];
    let mut stake_withdrawn_events = vec![];
    let mut stake_delegated_events = vec![];
    let mut stake_delegated_locked_events = vec![];
    let mut rebate_claimed_events = vec![];
    let mut delegation_parameters_updated_events = vec![];
    let mut rewards_assigned_events = vec![];
    let mut signalled_events = vec![];
    let mut burned_events = vec![];

    // Potentially consider adding log.index() to the IDs, to have them be truly unique in
    // transactions with potentially more than 1 of these messages
    for log in blk.logs() {
        if !(&Hex(&GRAPH_TOKEN_ADDRESS).to_string() == &Hex(&log.address()).to_string()
            || &Hex(&STAKING_CONTRACT).to_string() == &Hex(&log.address()).to_string()
            || &Hex(&REWARDS_MANAGER_CONTRACT).to_string() == &Hex(&log.address()).to_string()
            || &Hex(&GNS_CONTRACT).to_string() == &Hex(&log.address()).to_string()
            || &Hex(&CURATION_CONTRACT).to_string() == &Hex(&log.address()).to_string())
        {
            continue;
        }

        if let Some(event) = abi::erc20::events::Transfer::match_and_decode(log) {
            transfers.push(Transfer {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each Transfer needs a unique id
                from: event.from,
                to: event.to,
                value: event.value.to_string(), // Value is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeDeposited::match_and_decode(log) {
            stake_deposited_events.push(StakeDeposited {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeWithdrawn::match_and_decode(log) {
            stake_withdrawn_events.push(StakeWithdrawn {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::StakeDelegated::match_and_decode(log) {
            stake_delegated_events.push(StakeDelegated {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                delegator: event.delegator,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) =
            abi::staking::events::StakeDelegatedLocked::match_and_decode(log)
        {
            stake_delegated_locked_events.push(StakeDelegatedLocked {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                delegator: event.delegator,
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::RebateClaimed::match_and_decode(log) {
            rebate_claimed_events.push(RebateClaimed {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                delegated_tokens: event.delegation_fees.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) =
            abi::staking::events::DelegationParametersUpdated::match_and_decode(log)
        {
            delegation_parameters_updated_events.push(DelegationParametersUpdated {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                indexing_reward_cut: event.indexing_reward_cut.to_string(),
                query_fee_cut: event.query_fee_cut.to_string(),
                delegator_parameter_cooldown: event.cooldown_blocks.to_string(),
                block_number: blk.number,
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) =
            abi::rewards_manager::events::RewardsAssigned::match_and_decode(log)
        {
            rewards_assigned_events.push(RewardsAssigned {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                amount: event.amount.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::curation::events::Signalled::match_and_decode(log) {
            signalled_events.push(Signalled {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                curator: event.curator,
                subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                signal: event.signal.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                curation_tax: event.curation_tax.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::curation::events::Burned::match_and_decode(log) {
            burned_events.push(Burned {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                curator: event.curator,
                subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                tokens: event.tokens.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                signal: event.signal.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                ordinal: log.ordinal() as u64,
            });
        }
    }

    // GNS ones require a bit extra work, as they are 2 different versions of the name signal and burn
    // due to the subgraph ID change/migration from address+index to nft id
    // Might be a bit tricky to support them.

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
    events.rebate_claimed_events = Some(RebateClaimedEvents {
        rebate_claimed_events: rebate_claimed_events,
    });
    events.delegation_parameters_updated_events = Some(DelegationParametersUpdatedEvents {
        delegation_parameters_updated_events: delegation_parameters_updated_events,
    });
    events.rewards_assigned_events = Some(RewardsAssignedEvents {
        rewards_assigned_events: rewards_assigned_events,
    });
    events.signalled_events = Some(SignalledEvents {
        signalled_events: signalled_events,
    });
    events.burned_events = Some(BurnedEvents {
        burned_events: burned_events,
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
            generate_key_delegated_stake(&stake_delegated.delegator, &stake_delegated.indexer),
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
            generate_key(&stake_delegated.delegator),
            BigInt::from_str(&stake_delegated.tokens).unwrap(),
        );
    }
}

// Curator entities track the cumulative signalled amount, not the current amount
#[substreams::handlers::store]
fn store_cumulative_curator_signalled(events: Events, s: StoreAddBigInt) {
    let signalled_events = events.signalled_events.unwrap();

    for signalled in signalled_events.signalled_events {
        s.add(
            signalled.ordinal,
            generate_key(&signalled.curator),
            BigInt::from_str(&signalled.tokens)
                .unwrap()
                .sub(BigInt::from_str(&signalled.curation_tax).unwrap()),
        );
    }
}

// Curator entities track the cumulative burned amount, not the current amount
#[substreams::handlers::store]
fn store_cumulative_curator_burned(events: Events, s: StoreAddBigInt) {
    let burned_events = events.burned_events.unwrap();

    for burned in burned_events.burned_events {
        s.add(
            burned.ordinal,
            generate_key(&burned.curator),
            BigInt::from_str(&burned.tokens).unwrap(),
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

// GraphNetwork entity tracks the total signalled, not the cumulative amount separately
#[substreams::handlers::store]
fn store_total_signalled(store_changes: StorageChanges, s: StoreAddBigInt) {
    let curation_pools = store_changes.curation_pools.unwrap();

    for curation_pool in curation_pools.curation_pools {
        s.add(
            curation_pool.ordinal,
            "totalTokensSignalled",
            BigInt::from_str(&curation_pool.new_signal)
                .unwrap()
                .sub(BigInt::from_str(&curation_pool.old_signal).unwrap()),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_indexer(storage_changes: StorageChanges, s: StoreSetIfNotExistsString) {
    let indexer_stakes = storage_changes.indexer_stakes.unwrap();
    for indexer_stake in indexer_stakes.indexer_stakes {
        s.set_if_not_exists(
            indexer_stake.ordinal,
            generate_key(&indexer_stake.indexer),
            &generate_key(&indexer_stake.indexer),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_delegator(events: Events, s: StoreSetIfNotExistsString) {
    let stake_delegated_events = events.stake_delegated_events.unwrap();
    for stake_delegated in stake_delegated_events.stake_delegated_events {
        s.set_if_not_exists(
            stake_delegated.ordinal,
            generate_key_delegated_stake(&stake_delegated.delegator, &stake_delegated.indexer),
            &generate_key(&stake_delegated.delegator),
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
            generate_key(&delegation_parameters_updated.indexer),
            &delegation_parameters_updated,
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_curator(events: Events, s: StoreSetIfNotExistsString) {
    let signalled_events = events.signalled_events.unwrap();
    for signalled in signalled_events.signalled_events {
        s.set_if_not_exists(
            signalled.ordinal,
            generate_key(&signalled.curator),
            &generate_key(&signalled.curator),
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
    graph_account_curator_deltas: Deltas<DeltaString>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::grt_balance_change(grt_balance_deltas, &mut entity_changes);
    db::graph_account_indexer_change(graph_account_indexer_deltas, &mut entity_changes);
    db::graph_account_delegator_change(graph_account_delegator_deltas, &mut entity_changes);
    db::graph_account_curator_change(graph_account_curator_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_indexer_entities(
    storage_changes: StorageChanges,
    staked_token_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    let indexer_stakes = storage_changes.indexer_stakes.unwrap();
    db::indexer_stake_change(indexer_stakes, staked_token_deltas, &mut entity_changes);
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_delegated_stake_entities(
    cumulative_delegated_stake_deltas: Deltas<DeltaBigInt>,
    cumulative_delegator_stake_deltas: Deltas<DeltaBigInt>,
    total_delegated_stake_deltas: Deltas<DeltaBigInt>,
    storage_changes: StorageChanges,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    let delegation_pools = storage_changes.delegation_pools.unwrap();
    db::delegated_stake_change(
        cumulative_delegated_stake_deltas,
        cumulative_delegator_stake_deltas,
        total_delegated_stake_deltas,
        delegation_pools,
        &mut entity_changes,
    );
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn map_curator_entities(
    cumulative_curator_signalled_deltas: Deltas<DeltaBigInt>,
    cumulative_curator_burned_deltas: Deltas<DeltaBigInt>,
    total_signalled_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    db::curation_signal_change(
        cumulative_curator_signalled_deltas,
        cumulative_curator_burned_deltas,
        total_signalled_deltas,
        &mut entity_changes,
    );
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
    curator_entities: EntityChanges,
) -> Result<EntityChanges, substreams::errors::Error> {
    Ok(EntityChanges {
        entity_changes: [
            graph_network_entities.entity_changes,
            graph_account_entities.entity_changes,
            indexer_entities.entity_changes,
            delegated_stake_entities.entity_changes,
            curator_entities.entity_changes,
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
