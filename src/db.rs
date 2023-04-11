use crate::pb::erc20::{DelegationPools, IndexerStakes};
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, DeltaString, Deltas};
use substreams::Hex;
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

// --------------------
//  Map GRT Balances Entity Changes
// --------------------
pub fn grt_balance_change(grt_deltas: Deltas<DeltaBigInt>, entity_changes: &mut EntityChanges) {
    for delta in grt_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("balance", delta);
    }
}

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
                &generate_key(&indexer_stake.indexer),
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
            &generate_key(&delegation_pool.indexer),
            delegation_pool.ordinal,
            Operation::Update, // Update will create the entity if it does not exist
        )
        .change("delegatedTokens", BigInt::from_str(&delegation_pool.new_stake).unwrap());
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
//  Map Indexer Stake Entity Changes
// --------------------
pub fn graph_account_indexer_change(
    graph_account_indexer_deltas: Deltas<DeltaString>,
    entity_changes: &mut EntityChanges,
) {
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
}

// --------------------
//  Map Delegator Stake Entity Changes
// --------------------
pub fn graph_account_delegator_change(
    graph_account_delegator_deltas: Deltas<DeltaString>,
    entity_changes: &mut EntityChanges,
) {
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
}

// --------------------
//  Map Curator Entity Changes
// --------------------
pub fn graph_account_curator_change(
    graph_account_curator_deltas: Deltas<DeltaString>,
    entity_changes: &mut EntityChanges,
) {
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

// -------------------- KEY GENERATORS --------------------
fn generate_key(account: &Vec<u8>) -> String {
    return Hex(account).to_string();
}
