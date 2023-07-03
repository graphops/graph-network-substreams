use crate::pb::erc20::{
    CurationPools, DelegationPools, IndexerStakes, IndexingRewards, SubgraphAllocations, Events
};
use crate::utils;
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, DeltaString, Deltas};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

// --------------------
//  Map GRT Mint, Burn and Total Supply Entity Changes
// --------------------
pub fn grt_global_change(
    grt_global_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for delta in grt_global_deltas.deltas {
        let name = match delta.key.as_str() {
            "totalGRTBurned" => "totalGRTBurned",
            "totalGRTMinted" => "totalGRTMinted",
            "totalSupply" => "totalSupply",
            _ => {
                continue;
            }
        };
        entity_changes
            .push_change(
                "GraphNetwork",
                "1", // GraphNetwork has id 1
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(name, delta);
    }
}

// --------------------
//  Map Indexer Stake Entity Changes
// --------------------
pub fn indexer_stake_change(
    indexer_stakes: IndexerStakes,
    staked_token_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for delta in staked_token_deltas.deltas {
        entity_changes
            .push_change("GraphNetwork", "1", delta.ordinal, Operation::Update)
            .change("totalTokensStaked", delta);
    }
    for indexer_stake in indexer_stakes.indexer_stakes {
        entity_changes
            .push_change(
                "Indexer",
                &utils::generate_key(&indexer_stake.indexer),
                indexer_stake.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "stakedTokens",
                BigInt::from_str(&indexer_stake.new_stake).unwrap(),
            );
    }
}

// --------------------
//  Map Delegated Stake Entity Changes
// --------------------
pub fn delegated_stake_change(
    cumulative_delegated_stake_deltas: Deltas<DeltaBigInt>,
    cumulative_delegator_stake_deltas: Deltas<DeltaBigInt>,
    total_delegated_stake_deltas: Deltas<DeltaBigInt>,
    delegation_pools: DelegationPools,
    entity_changes: &mut EntityChanges,
) {
    for delta in cumulative_delegated_stake_deltas.deltas {
        entity_changes
            .push_change(
                "DelegatedStake",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("stakedTokens", &delta);
    }

    for delta in cumulative_delegator_stake_deltas.deltas {
        entity_changes
            .push_change(
                "Delegator",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("totalStakedTokens", &delta);
    }

    for delta in total_delegated_stake_deltas.deltas {
        entity_changes
            .push_change(
                "GraphNetwork",
                "1",
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("totalTokensDelegated", &delta);
    }

    for delegation_pool in delegation_pools.delegation_pools {
        entity_changes
            .push_change(
                "Indexer",
                &utils::generate_key(&delegation_pool.indexer),
                delegation_pool.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "delegatedTokens",
                BigInt::from_str(&delegation_pool.new_stake).unwrap(),
            );
    }
}

// --------------------
//  Map Curation Entity Changes
// --------------------
pub fn curation_signal_change(
    cumulative_curator_signalled_deltas: Deltas<DeltaBigInt>,
    cumulative_curator_burned_deltas: Deltas<DeltaBigInt>,
    total_signalled_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for delta in cumulative_curator_signalled_deltas.deltas {
        entity_changes
            .push_change(
                "Curator",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("totalSignalledTokens", &delta);
    }

    for delta in cumulative_curator_burned_deltas.deltas {
        entity_changes
            .push_change(
                "Curator",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("totalUnsignalledTokens", &delta);
    }

    for delta in total_signalled_deltas.deltas {
        entity_changes
            .push_change(
                "GraphNetwork",
                "1",
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("totalTokensSignalled", &delta);
    }
}

// --------------------
//  Map Curator Entity Changes
// --------------------
pub fn graph_account_change(
    grt_balance_deltas: Deltas<DeltaBigInt>,
    graph_account_indexer_deltas: Deltas<DeltaString>,
    graph_account_delegator_deltas: Deltas<DeltaString>,
    graph_account_curator_deltas: Deltas<DeltaString>,
    entity_changes: &mut EntityChanges,
) {
    for delta in grt_balance_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("balance", delta);
    }
    for delta in graph_account_indexer_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("indexer", &delta.key);
        entity_changes
            .push_change(
                "Indexer",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("account", &delta.key);
    }
    for delta in graph_account_delegator_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "delegator",
                &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
            );
        entity_changes
            .push_change(
                "Delegator",
                &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "account",
                &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
            );
        entity_changes
            .push_change(
                "DelegatedStake",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "indexer",
                &delta.key.as_str().split(":").last().unwrap().to_string(),
            )
            .change(
                "delegator",
                &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
            );
    }
    for delta in graph_account_curator_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("curator", &delta.key);
        entity_changes
            .push_change(
                "Curator",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("account", &delta.key);
    }
}

pub fn subgraph_deployment_change(
    subgraph_allocations: SubgraphAllocations,
    curation_pools: CurationPools,
<<<<<<< HEAD
    subgraph_deployment_rewards_deltas: Deltas<DeltaBigInt>,
=======
    indexing_rewards: IndexingRewards,
>>>>>>> d432184 (feat: query fees added)
    curator_fee_rewards_deltas: Deltas<DeltaBigInt>,
    signal_amount_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for subgraph_allocation in subgraph_allocations.subgraph_allocations {
        entity_changes
            .push_change(
                "SubgraphDeployment",
                &subgraph_allocation.subgraph_deployment_id,
                subgraph_allocation.ordinal,
                Operation::Update,
            )
            .change(
                "stakedTokens",
                BigInt::from_str(&subgraph_allocation.new_tokens).unwrap(),
            );
    }
    for curation_pool in curation_pools.curation_pools {
        entity_changes
            .push_change(
                "SubgraphDeployment",
                &curation_pool.subgraph_deployment_id,
                curation_pool.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "signalledTokens",
                BigInt::from_str(&curation_pool.new_signal).unwrap(),
            );
    }

    // just realized I need to add a store to aggregate indexer rewards per subgraph deployment since they are for all indexers 
    // this code needs to be updated. db should recive some stores, not indexing rewards directly. allocations should use indexing rewards directly
    for delta in subgraph_deployment_rewards_deltas.deltas {
        let name = match delta.key.as_str().split(":").last().unwrap() {
            "indexingRewardAmount" => "indexingRewardAmount",
            "indexingIndexerRewardAmount" => "indexingIndexerRewardAmount",
            "indexingDelegatorRewardAmount" => "indexingDelegatorRewardAmount",
            _ => {
                continue;
            }
        };
        entity_changes
            .push_change(
                "SubgraphDeployment",
                &delta.key.as_str().split(":").nth(0).unwrap(), 
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(name, delta);
    }

    for delta in curator_fee_rewards_deltas.deltas {
        entity_changes
            .push_change(
                "SubgraphDeployment",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "curatorFeeRewards",
                &delta,
            );
    }
    for delta in signal_amount_deltas.deltas {
        entity_changes
            .push_change(
                "SubgraphDeployment",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "signalAmount",
                &delta,
            );
    }
}
pub fn allocation_change(
    events: Events,
    indexing_rewards:IndexingRewards,
    curator_rewards_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    let allocation_created_events = events.allocation_created_events.unwrap();
    let allocation_closed_events = events.allocation_closed_events.unwrap();

    for allocation_created in allocation_created_events.allocation_created_events {
        entity_changes
            .push_change(
                "Allocation",
                &utils::generate_key(&allocation_created.allocation_id),
                allocation_created.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("indexer", &allocation_created.indexer)
            .change("creator", &allocation_created.creator)
            .change("activeForIndexer", &allocation_created.indexer)
            .change("subgraphDeploymentId", allocation_created.subgraph_deployment_id)
            .change(
                "allocatedTokens",
                allocation_created.tokens,
            );
    }
    for allocation_closed in allocation_closed_events.allocation_closed_events {
        entity_changes
            .push_change(
                "Allocation",
                &utils::generate_key(&allocation_closed.allocation_id),
                allocation_closed.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            ).change(
                "effectiveAllocation",
                allocation_closed.effective_allocation,
            );
    }
    for indexing_reward in indexing_rewards.indexing_rewards {
        entity_changes
            .push_change(
                "Allocation",
                &indexing_reward.allocation_id,
                indexing_reward.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("indexingRewards", indexing_reward.amount)
            .change(
                "indexingIndexerRewards",
                indexing_reward.indexer_rewards,
            )
            .change(
                "indexingDelegatorRewards",
                indexing_reward.delegator_rewards,
            );
    }
    for delta in curator_rewards_deltas.deltas {
        entity_changes
        .push_change(
            "Allocation",
            &delta.key,
            delta.ordinal,
            Operation::Update, // Update will create the entity if it does not exist
        ).change(
            "curatorRewards",
            &delta,
        );
    }

}

pub fn query_fee_rebate_change(
    query_fee_rebate_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,

) {
    for delta in query_fee_rebate_deltas.deltas {
        if  &delta.key.as_str().split(":").nth(0).unwrap() == &"SubgraphDeployment"{
            entity_changes
            .push_change(
                "SubgraphDeployment",
                &delta.key.as_str().split(":").last().unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "queryFeeRebates",
                &delta,
            );
        }    
        else if  &delta.key.as_str().split(":").nth(0).unwrap() == &"Allocation"{
            entity_changes
            .push_change(
                "Allocation",
                &delta.key.as_str().split(":").last().unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "queryFeeRebates",
                &delta,
            );
        }    
    }
}
pub fn query_fees_change(
    query_fees_amount_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,

) {
    for delta in query_fees_amount_deltas.deltas {
        if  &delta.key.as_str().split(":").nth(0).unwrap() == &"SubgraphDeployment"{
            entity_changes
            .push_change(
                "SubgraphDeployment",
                &delta.key.as_str().split(":").last().unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "queryFeesAmount",
                &delta,
            );
        }    
        else if  &delta.key.as_str().split(":").nth(0).unwrap() == &"Allocation"{
            entity_changes
            .push_change(
                "Allocation",
                &delta.key.as_str().split(":").last().unwrap().to_string(),
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change(
                "queryFeesCollected",
                &delta,
            );
        }    
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> d432184 (feat: query fees added)
