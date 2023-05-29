use crate::db;
use crate::pb::erc20::*;

use substreams::store::{DeltaBigInt, DeltaString, Deltas};

use substreams_entity_change::pb::entity::EntityChanges;

// -------------------- GRAPH_OUT --------------------
// Final map for executing all entity change maps together
// Run this map to check the health of the entire substream

#[substreams::handlers::map]
pub fn graph_out(
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
    indexing_rewards: IndexingRewards,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut graph_network_entity_changes: EntityChanges = Default::default();
    db::grt_global_change(grt_global_deltas, &mut graph_network_entity_changes);

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
        subgraph_allocations,
        curation_pools,
        indexing_rewards,
        &mut subgraph_deployment_entity_changes,
    );

    Ok(EntityChanges {
        entity_changes: [
            graph_network_entity_changes.entity_changes,
            graph_account_entity_changes.entity_changes,
            indexer_entity_changes.entity_changes,
            delegated_stake_entity_changes.entity_changes,
            curator_entity_changes.entity_changes,
            subgraph_deployment_entity_changes.entity_changes,
        ]
        .concat(),
    })
}
