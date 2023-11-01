use crate::db;
use crate::pb::erc20::*;

use substreams::{
    pb::substreams::Clock,
    store::{DeltaBigInt, DeltaString, Deltas},
};
use substreams_entity_change::pb::entity::EntityChanges;

// -------------------- GRAPH_OUT --------------------
// Final map for executing all entity change maps together
// Run this map to check the health of the entire substream

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    events: Events,
    epoch_start_deltas: Deltas<DeltaString>,
    epoch_end_deltas: Deltas<DeltaString>,
    epoch_signal_deltas: Deltas<DeltaBigInt>,
    epoch_stake_deltas: Deltas<DeltaBigInt>,
    epoch_rewards_deltas: Deltas<DeltaBigInt>,
    grt_global_deltas: Deltas<DeltaBigInt>,
    grt_balance_deltas: Deltas<DeltaBigInt>,
    graph_account_indexer_deltas: Deltas<DeltaString>,
    graph_account_delegator_deltas: Deltas<DeltaString>,
    graph_account_curator_deltas: Deltas<DeltaString>,
    storage_changes: StorageChanges,
    staked_token_deltas: Deltas<DeltaBigInt>,
    cumulative_delegated_stake_deltas: Deltas<DeltaBigInt>,
    cumulative_delegator_stake_deltas: Deltas<DeltaBigInt>,
    total_delegated_stake_deltas: Deltas<DeltaBigInt>,
    cumulative_curator_signalled_deltas: Deltas<DeltaBigInt>,
    cumulative_curator_burned_deltas: Deltas<DeltaBigInt>,
    total_signalled_deltas: Deltas<DeltaBigInt>,
    query_fee_rebate_deltas: Deltas<DeltaBigInt>,
    query_fees_amount_deltas: Deltas<DeltaBigInt>,
    curator_fee_rewards_deltas: Deltas<DeltaBigInt>,
    curator_rewards_deltas: Deltas<DeltaBigInt>,
    signal_amount_deltas: Deltas<DeltaBigInt>,
    subgraph_deployment_rewards_deltas: Deltas<DeltaBigInt>,
    subgraph_deployment_ipfs_hash_deltas: Deltas<DeltaString>,
    indexing_rewards: IndexingRewards,
    version_count_deltas: Deltas<DeltaBigInt>,
    version_deltas: Deltas<DeltaString>,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut graph_network_entity_changes: EntityChanges = Default::default();
    db::graph_network_change(
        grt_global_deltas,
        events.clone(),
        &mut graph_network_entity_changes,
        clock.number == 11446764, //hard-coded, block number where graph-network entity is created
    );

    let mut graph_account_entity_changes: EntityChanges = Default::default();
    db::graph_account_change(
        grt_balance_deltas,
        graph_account_indexer_deltas,
        graph_account_delegator_deltas,
        graph_account_curator_deltas,
        &mut graph_account_entity_changes,
    );

    let mut indexer_entity_changes: EntityChanges = Default::default();
    let indexer_stakes = storage_changes.indexer_stakes.unwrap();
    db::indexer_stake_change(
        indexer_stakes,
        staked_token_deltas,
        &mut indexer_entity_changes,
    );

    let mut delegated_stake_entity_changes: EntityChanges = Default::default();
    let delegation_pools = storage_changes.delegation_pools.unwrap();
    db::delegated_stake_change(
        cumulative_delegated_stake_deltas,
        cumulative_delegator_stake_deltas,
        total_delegated_stake_deltas,
        delegation_pools,
        &mut delegated_stake_entity_changes,
    );

    let mut curator_entity_changes: EntityChanges = Default::default();
    db::curation_signal_change(
        cumulative_curator_signalled_deltas,
        cumulative_curator_burned_deltas,
        total_signalled_deltas,
        &mut curator_entity_changes,
    );

    let mut subgraph_deployment_entity_changes: EntityChanges = Default::default();
    let subgraph_allocations = storage_changes.subgraph_allocations.unwrap();
    let curation_pools = storage_changes.curation_pools.unwrap();
    db::subgraph_deployment_change(
        events.clone(),
        subgraph_allocations,
        curation_pools,
        subgraph_deployment_rewards_deltas,
        curator_fee_rewards_deltas,
        signal_amount_deltas,
        subgraph_deployment_ipfs_hash_deltas,
        &mut subgraph_deployment_entity_changes,
    );

    let mut allocation_entity_changes: EntityChanges = Default::default();
    db::allocation_change(
        events,
        indexing_rewards,
        curator_rewards_deltas,
        &mut allocation_entity_changes,
    );

    let mut query_fee_rebate_changes: EntityChanges = Default::default();
    db::query_fee_rebate_change(
        query_fee_rebate_deltas.clone(),
        &mut query_fee_rebate_changes,
    );

    let mut query_fee_changes: EntityChanges = Default::default();
    db::query_fees_change(query_fees_amount_deltas, &mut query_fee_changes);

    let mut epoch_changes: EntityChanges = Default::default();
    db::epoch_change(
        epoch_start_deltas,
        epoch_end_deltas,
        epoch_signal_deltas,
        epoch_stake_deltas,
        query_fee_rebate_deltas,
        epoch_rewards_deltas,
        &mut epoch_changes,
    );

    let mut subgraph_changes: EntityChanges = Default::default();
    db::subgraph_change(version_count_deltas, &mut subgraph_changes);

    let mut version_changes: EntityChanges = Default::default();
    db::version_change(version_deltas, &mut version_changes);

    Ok(EntityChanges {
        entity_changes: [
            graph_network_entity_changes.entity_changes,
            graph_account_entity_changes.entity_changes,
            indexer_entity_changes.entity_changes,
            delegated_stake_entity_changes.entity_changes,
            curator_entity_changes.entity_changes,
            subgraph_deployment_entity_changes.entity_changes,
            allocation_entity_changes.entity_changes,
            query_fee_rebate_changes.entity_changes,
            query_fee_changes.entity_changes,
            epoch_changes.entity_changes,
            subgraph_changes.entity_changes,
            version_changes.entity_changes,
        ]
        .concat(),
    })
}
