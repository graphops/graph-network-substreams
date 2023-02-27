use substreams::store::{DeltaBigInt, DeltaString, Deltas};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use substreams::{Hex};

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
    indexer_stake_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for delta in indexer_stake_deltas.deltas {
        if delta.key == "totalTokensStaked" {
            entity_changes
                .push_change(
                    "GraphNetwork",
                    "1",
                    delta.ordinal,
                    Operation::Update, // Update will create the entity if it does not exist
                )
                .change("totalTokensStaked", delta);
        } else {
            entity_changes
                .push_change(
                    "Indexer",
                    &delta.key,
                    delta.ordinal,
                    Operation::Update, // Update will create the entity if it does not exist
                )
                .change("stakedTokens", delta);
        }
    }
}

// --------------------
//  Map Delegated Stake Entity Changes
// --------------------
pub fn delegated_stake_change(
    delegated_stake_deltas: Deltas<DeltaBigInt>,
    entity_changes: &mut EntityChanges,
) {
    for delta in delegated_stake_deltas.deltas {
        if delta.key == "totalTokensDelegated" {
            entity_changes
                .push_change(
                    "GraphNetwork",
                    "1",
                    delta.ordinal,
                    Operation::Update, // Update will create the entity if it does not exist
                )
                .change("totalTokensDelegated", delta);
        } else {
            entity_changes
                .push_change(
                    "DelegatedStake",
                    &delta.key,
                    delta.ordinal,
                    Operation::Update, // Update will create the entity if it does not exist
                )
                .change("stakedTokens", &delta);
            entity_changes
                .push_change(
                    "Delegator",
                    &delta.key.as_str().split(":").nth(0).unwrap().to_string(),
                    delta.ordinal,
                    Operation::Update, // Update will create the entity if it does not exist
                ).change("totalStakedTokens", delta);
        }
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
            .change("Indexer", &delta.key);
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
//  Map Indexer Stake Entity Changes
// --------------------
pub fn graph_account_delegator_change(
    graph_account_delegator_deltas: Deltas<DeltaString>,
    entity_changes: &mut EntityChanges,
) {
    for delta in graph_account_delegator_deltas.deltas {
        entity_changes
            .push_change(
                "GraphAccount",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("Delegator", &delta.key);
        entity_changes
            .push_change(
                "Delegator",
                &delta.key,
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            )
            .change("account", &delta.key);
        entity_changes
            .push_change(
                "DelegatedStake",
                &generate_key_delegated_stake(&delta.key, &delta.new_value), 
                delta.ordinal,
                Operation::Update, // Update will create the entity if it does not exist
            ).change("indexer", &delta.new_value)
             .change("delegator", &delta.key);
    }
}

fn generate_key_delegated_stake(delegator: &String, indexer: &String) -> String {
    return format!("{}:{}", Hex(delegator).to_string(), Hex(indexer).to_string())
}
