use substreams::store::{DeltaBigInt, DeltaString, Deltas};
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
                .change("account", &delta.key)
                .change("stakedTokens", delta);
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
    }
}
