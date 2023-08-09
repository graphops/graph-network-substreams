use crate::abi;
use crate::pb::erc20::*;
use crate::utils;
use std::ops::Sub;
use substreams::errors::Error;
use substreams::scalar::BigInt;
use substreams::{hex, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

// Contract Addresses
const GRAPH_TOKEN_ADDRESS: [u8; 20] = hex!("c944E90C64B2c07662A292be6244BDf05Cda44a7");
const STAKING_CONTRACT: [u8; 20] = hex!("F55041E37E12cD407ad00CE2910B8269B01263b9");
const REWARDS_MANAGER_CONTRACT: [u8; 20] = hex!("9Ac758AB77733b4150A901ebd659cbF8cB93ED66");
const GNS_CONTRACT: [u8; 20] = hex!("aDcA0dd4729c8BA3aCf3E99F3A9f471EF37b6825");
const CURATION_CONTRACT: [u8; 20] = hex!("8FE00a685Bcb3B2cc296ff6FfEaB10acA4CE1538");

// -------------------- INITIAL MAPS --------------------
#[substreams::handlers::map]
fn map_storage_changes(blk: eth::Block) -> Result<StorageChanges, Error> {
    let mut storage_changes = StorageChanges::default();
    let mut indexer_stakes = vec![];
    let mut delegation_pools = vec![];
    let mut curation_pools = vec![];
    let mut subgraph_allocations = vec![];
    let mut delegator_rewards = vec![];

    for trx in blk.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            if let Some(event) = abi::staking::events::StakeDeposited::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 14, 0) {
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
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 14, 0) {
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
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 20, 2) {
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
            if let Some(event) = abi::staking::events::StakeDelegatedLocked::match_and_decode(&log)
            {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 20, 2) {
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
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 20, 2) {
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
            if let Some(event) = abi::staking::events::AllocationClosed::match_and_decode(&log) {
                // We track the AllocationClosed event instead of RewardsAssigned here. I think it is because their calls are different
                // when their transaction is the same. And we have the call data here, so we need to use the event that shares the same call
                // with the storage_change
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key == utils::find_key(&event.indexer, 20, 2) {
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
                            });
                            // indexingDelegatorRewards are distributed whenever an allocation is closed
                            delegator_rewards.push(DelegatorReward {
                                id: Hex(&trx.hash).to_string(),
                                allocation_id: event.allocation_id.to_vec(),
                                rewards: BigInt::from_unsigned_bytes_be(&storage_change.new_value)
                                    .sub(BigInt::from_signed_bytes_be(&storage_change.old_value))
                                    .to_string(),
                                ordinal: log.ordinal,
                            });
                        }
                    }
                }
            }
            if let Some(event) = abi::curation::events::Signalled::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&CURATION_CONTRACT) {
                        if storage_change.key
                            == utils::find_key(&event.subgraph_deployment_id, 15, 0)
                        {
                            curation_pools.push(CurationPool {
                                id: Hex(&trx.hash).to_string(),
                                subgraph_deployment_id: Hex(&event.subgraph_deployment_id)
                                    .to_string(),
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
            if let Some(event) = abi::curation::events::Burned::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&CURATION_CONTRACT) {
                        if storage_change.key
                            == utils::find_key(&event.subgraph_deployment_id, 15, 0)
                        {
                            curation_pools.push(CurationPool {
                                id: Hex(&trx.hash).to_string(),
                                subgraph_deployment_id: Hex(&event.subgraph_deployment_id)
                                    .to_string(),
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
            if let Some(event) = abi::staking::events::AllocationCollected::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&CURATION_CONTRACT) {
                        if storage_change.key
                            == utils::find_key(&event.subgraph_deployment_id, 15, 0)
                        {
                            curation_pools.push(CurationPool {
                                id: Hex(&trx.hash).to_string(),
                                subgraph_deployment_id: Hex(&event.subgraph_deployment_id)
                                    .to_string(),
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
            if let Some(event) = abi::gns::events::SubgraphPublished::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key
                            == utils::find_key(&event.subgraph_deployment_id, 16, 0)
                        {
                            subgraph_allocations.push(SubgraphAllocation {
                                id: Hex(&trx.hash).to_string(),
                                subgraph_deployment_id: Hex(&event.subgraph_deployment_id)
                                    .to_string(),
                                new_tokens: BigInt::from_unsigned_bytes_be(
                                    &storage_change.new_value,
                                )
                                .into(),
                                old_tokens: BigInt::from_unsigned_bytes_be(
                                    &storage_change.old_value,
                                )
                                .into(),
                                ordinal: log.ordinal,
                            })
                        }
                    }
                }
            }
            if let Some(event) = abi::staking::events::AllocationCreated::match_and_decode(&log) {
                for storage_change in &call_view.call.storage_changes {
                    if storage_change.address.eq(&STAKING_CONTRACT) {
                        if storage_change.key
                            == utils::find_key(&event.subgraph_deployment_id, 16, 0)
                        {
                            subgraph_allocations.push(SubgraphAllocation {
                                id: Hex(&trx.hash).to_string(),
                                subgraph_deployment_id: Hex(&event.subgraph_deployment_id)
                                    .to_string(),
                                new_tokens: BigInt::from_unsigned_bytes_be(
                                    &storage_change.new_value,
                                )
                                .into(),
                                old_tokens: BigInt::from_unsigned_bytes_be(
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

    //indexer_stakes.sort_by(|x, y| x.ordinal.cmp(&y.ordinal));
    storage_changes.indexer_stakes = Some(IndexerStakes {
        indexer_stakes: indexer_stakes,
    });
    storage_changes.delegation_pools = Some(DelegationPools {
        delegation_pools: delegation_pools,
    });
    storage_changes.delegator_rewards = Some(DelegatorRewards {
        delegator_rewards: delegator_rewards,
    });
    storage_changes.curation_pools = Some(CurationPools {
        curation_pools: curation_pools,
    });
    storage_changes.subgraph_allocations = Some(SubgraphAllocations {
        subgraph_allocations: subgraph_allocations,
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
    let mut allocation_created_events = vec![];
    let mut allocation_closed_events = vec![];
    let mut allocation_collected_events = vec![];

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
            rebate_claimed_events.push(
                //missing epoch and forEpoch fields since they are not yet needed
                RebateClaimed {
                    id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                    indexer: event.indexer,
                    subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                    allocation_id: event.allocation_id,
                    tokens: event.tokens.to_string(),
                    delegation_fees: event.delegation_fees.to_string(), // Tokens is origanally BigInt but proto does not have BigInt so we use string
                    ordinal: log.ordinal() as u64,
                },
            );
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
        } else if let Some(event) = abi::staking::events::AllocationCreated::match_and_decode(log) {
            allocation_created_events.push(AllocationCreated {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer.clone(),
                creator: log.receipt.transaction.from.clone(), //migh also be the operator, need to update
                active_for: event.indexer, //need to find a way to make this null later
                subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                created_at_epoch: event.epoch.to_string(),
                created_at_block_hash: Hex(&blk.hash).to_string(),
                created_at_block_number: blk.number.to_string(),
                created_at: blk.header.clone().unwrap().timestamp.unwrap().to_string(),
                tokens: event.tokens.to_string(),
                allocation_id: event.allocation_id.to_vec(),
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::AllocationClosed::match_and_decode(log) {
            allocation_closed_events.push(AllocationClosed {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                closed_at_epoch: event.epoch.to_string(),
                closed_at_block_hash: Hex(&blk.hash).to_string(),
                closed_at_block_number: blk.number.to_string(),
                closed_at: blk.header.clone().unwrap().timestamp.unwrap().to_string(),
                tokens: event.tokens.to_string(),
                allocation_id: event.allocation_id.to_vec(),
                effective_allocation: event.effective_allocation.to_string(),
                poi: Hex(&event.poi).to_string(),
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) = abi::staking::events::AllocationCollected::match_and_decode(log)
        {
            allocation_collected_events.push(AllocationCollected {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                subgraph_deployment_id: event.subgraph_deployment_id.to_vec(),
                epoch: event.epoch.to_string(),
                tokens: event.tokens.to_string(),
                allocation_id: event.allocation_id.to_vec(),
                curation_fees: event.curation_fees.to_string(),
                rebate_fees: event.rebate_fees.to_string(),
                ordinal: log.ordinal() as u64,
            });
        } else if let Some(event) =
            abi::rewards_manager::events::RewardsAssigned::match_and_decode(log)
        {
            rewards_assigned_events.push(RewardsAssigned {
                id: Hex(&log.receipt.transaction.hash).to_string(), // Each event needs a unique id
                indexer: event.indexer,
                allocation_id: event.allocation_id.to_vec(),
                epoch: event.epoch.to_string(),
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
    events.allocation_created_events = Some(AllocationCreatedEvents {
        allocation_created_events: allocation_created_events,
    });
    events.allocation_closed_events = Some(AllocationClosedEvents {
        allocation_closed_events: allocation_closed_events,
    });
    events.allocation_collected_events = Some(AllocationCollectedEvents {
        allocation_collected_events: allocation_collected_events,
    });

    Ok(events)
}
