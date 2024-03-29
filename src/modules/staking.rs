use crate::pb::erc20::*;
use crate::utils;
use std::ops::Sub;
use std::str::FromStr;
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams::{
    store::StoreAddBigInt, store::StoreSetIfNotExists, store::StoreSetIfNotExistsString,
    store::StoreSetProto,
};

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

#[substreams::handlers::store]
fn store_epoch_stake(events: Events, store: StoreGetBigInt, s: StoreAddBigInt) {
    let stake_deposited_events = events.stake_deposited_events.unwrap();
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::zero() {
                let current_epoch = epoch_count.sub(1).to_string();
                for stake_deposited in stake_deposited_events.stake_deposited_events {
                    s.add(
                        stake_deposited.ordinal,
                        &current_epoch,
                        BigInt::from_str(&stake_deposited.tokens).unwrap(),
                    );
                }
            }
        }
        None => (),
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

#[substreams::handlers::store]
fn store_subgraph_deployment_id(events: Events, s: StoreSetString) {
    let allocation_created_events = events.allocation_created_events.unwrap();

    for allocation_created in allocation_created_events.allocation_created_events {
        s.set(
            allocation_created.ordinal,
            Hex(&allocation_created.allocation_id).to_string(),
            &Hex(&allocation_created.subgraph_deployment_id).to_string(),
        );
    }
}

#[substreams::handlers::store]
fn store_subgraph_deployment_ipfs_hash(events: Events, s: StoreSetIfNotExistsString) {
    let allocation_created_events = events.allocation_created_events.unwrap();

    for allocation_created in allocation_created_events.allocation_created_events {
        s.set_if_not_exists(
            allocation_created.ordinal,
            &Hex(&allocation_created.subgraph_deployment_id).to_string(),
            &utils::generate_ipfs_hash(Hex(&allocation_created.subgraph_deployment_id).to_string()),
        );
    }
}

#[substreams::handlers::store]
fn store_query_fee_rebates(events: Events, store: StoreGetBigInt, s: StoreAddBigInt) {
    let rebate_claimed_events = events.rebate_claimed_events.unwrap();
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::zero() {
                let current_epoch = epoch_count.sub(1).to_string();
                for rebate_claimed in rebate_claimed_events.rebate_claimed_events.clone() {
                    s.add(
                        rebate_claimed.ordinal,
                        &current_epoch,
                        BigInt::from_str(&rebate_claimed.tokens).unwrap(),
                    );
                }
            }
        }
        None => (),
    }

    for rebate_claimed in rebate_claimed_events.rebate_claimed_events {
        s.add(
            rebate_claimed.ordinal,
            utils::generate_key_query_fee_rebates(
                "SubgraphDeployment".to_string(),
                &rebate_claimed.subgraph_deployment_id,
            )
            .to_string(),
            BigInt::from_str(&rebate_claimed.tokens).unwrap(),
        );
        s.add(
            rebate_claimed.ordinal,
            utils::generate_key_query_fee_rebates(
                "Allocation".to_string(),
                &rebate_claimed.allocation_id,
            )
            .to_string(),
            BigInt::from_str(&rebate_claimed.tokens).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_query_fees_amount(events: Events, s: StoreAddBigInt) {
    let allocation_collected_events = events.allocation_collected_events.unwrap();

    for allocation_collected in allocation_collected_events.allocation_collected_events {
        s.add(
            allocation_collected.ordinal,
            utils::generate_key_query_fee_rebates(
                "SubgraphDeployment".to_string(),
                &allocation_collected.subgraph_deployment_id,
            )
            .to_string(),
            BigInt::from_str(&allocation_collected.rebate_fees).unwrap(),
        );
        s.add(
            allocation_collected.ordinal,
            utils::generate_key_query_fee_rebates(
                "Allocation".to_string(),
                &allocation_collected.allocation_id,
            )
            .to_string(),
            BigInt::from_str(&allocation_collected.rebate_fees).unwrap(),
        );
    }
}
#[substreams::handlers::store]
fn store_curator_fee_rewards(events: Events, s: StoreAddBigInt) {
    let allocation_collected_events = events.allocation_collected_events.unwrap();

    for allocation_collected in allocation_collected_events.allocation_collected_events {
        s.add(
            allocation_collected.ordinal,
            Hex(&allocation_collected.subgraph_deployment_id).to_string(),
            BigInt::from_str(&allocation_collected.curation_fees).unwrap(),
        );
    }
}
#[substreams::handlers::store]
fn store_curator_rewards(events: Events, s: StoreAddBigInt) {
    let allocation_collected_events = events.allocation_collected_events.unwrap();

    for allocation_collected in allocation_collected_events.allocation_collected_events {
        s.add(
            allocation_collected.ordinal,
            Hex(&allocation_collected.allocation_id).to_string(),
            BigInt::from_str(&allocation_collected.curation_fees).unwrap(),
        );
    }
}
#[substreams::handlers::store]
fn store_subgraph_deployment_rewards(indexing_rewards: IndexingRewards, s: StoreAddBigInt) {
    for indexing_rewards in indexing_rewards.indexing_rewards {
        s.add(
            indexing_rewards.ordinal,
            utils::generate_key_indexing_rewards(
                indexing_rewards.subgraph_deployment_id.clone(),
                "indexingRewardAmount".to_string(),
            ),
            BigInt::from_str(&indexing_rewards.amount).unwrap(),
        );
        s.add(
            indexing_rewards.ordinal,
            utils::generate_key_indexing_rewards(
                indexing_rewards.subgraph_deployment_id.clone(),
                "indexingIndexerRewardAmount".to_string(),
            ),
            BigInt::from_str(&indexing_rewards.indexer_rewards).unwrap(),
        );
        s.add(
            indexing_rewards.ordinal,
            utils::generate_key_indexing_rewards(
                indexing_rewards.subgraph_deployment_id,
                "indexingDelegatorRewardAmount".to_string(),
            ),
            BigInt::from_str(&indexing_rewards.delegator_rewards).unwrap(),
        );
    }
}
#[substreams::handlers::store]
fn store_epoch_rewards(
    indexing_rewards: IndexingRewards,
    store: StoreGetBigInt,
    s: StoreAddBigInt,
) {
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::zero() {
                let current_epoch = epoch_count.sub(1).to_string();
                for indexing_rewards in indexing_rewards.indexing_rewards {
                    s.add(
                        indexing_rewards.ordinal,
                        utils::concat(
                            current_epoch.clone().to_string(),
                            "totalRewards".to_string(),
                        ),
                        BigInt::from_str(&indexing_rewards.amount).unwrap(),
                    );
                    s.add(
                        indexing_rewards.ordinal,
                        utils::concat(
                            current_epoch.clone().to_string(),
                            "totalIndexerRewards".to_string(),
                        ),
                        BigInt::from_str(&indexing_rewards.indexer_rewards).unwrap(),
                    );
                    s.add(
                        indexing_rewards.ordinal,
                        utils::concat(
                            current_epoch.clone().to_string(),
                            "totalDelegatorRewards".to_string(),
                        ),
                        BigInt::from_str(&indexing_rewards.delegator_rewards).unwrap(),
                    );
                }
            }
        }
        None => (),
    }
}

#[substreams::handlers::map]
fn map_indexing_rewards(
    storage_changes: StorageChanges,
    events: Events,
    store: StoreGetString,
) -> Result<IndexingRewards, Error> {
    let mut indexing_rewards = IndexingRewards::default();
    let mut indexing_rewards_vec = vec![];
    let delegator_rewards = storage_changes.delegator_rewards.unwrap();
    let rewards_assigned_events = events.rewards_assigned_events.unwrap();
    for rewards_assigned in rewards_assigned_events.rewards_assigned_events {
        let subgraph_deployment_id = store
            .get_at(
                rewards_assigned.ordinal,
                Hex(&rewards_assigned.allocation_id).to_string(),
            )
            .unwrap();
        indexing_rewards_vec.push(IndexingReward {
            id: rewards_assigned.id.clone(), // Each event needs a unique id
            indexer: rewards_assigned.clone().indexer,
            subgraph_deployment_id: subgraph_deployment_id,
            allocation_id: Hex(&rewards_assigned.allocation_id).to_string(),
            amount: rewards_assigned.clone().amount, // Tokens is origanally BigInt but proto does not have BigInt so we use string
            indexer_rewards: rewards_assigned.amount.to_string(),
            delegator_rewards: "0".to_string(),
            ordinal: rewards_assigned.ordinal,
        });
        for delegator_reward in &delegator_rewards.delegator_rewards {
            if rewards_assigned.allocation_id == delegator_reward.allocation_id {
                let target_id = rewards_assigned.clone().id;
                for indexing_reward in &mut indexing_rewards_vec {
                    if indexing_reward.id == target_id {
                        let delegator_rewards =
                            BigInt::from_str(&delegator_reward.rewards).unwrap();
                        let indexer_rewards = BigInt::from_str(&rewards_assigned.clone().amount)
                            .unwrap()
                            .sub(delegator_rewards.clone());
                        indexing_reward.delegator_rewards = delegator_rewards.to_string();
                        indexing_reward.indexer_rewards = indexer_rewards.to_string();
                    }
                    break;
                }
                break;
            }
        }
    }
    indexing_rewards.indexing_rewards = indexing_rewards_vec;
    Ok(indexing_rewards)
}
