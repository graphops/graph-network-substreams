use crate::pb::erc20::*;
use crate::utils;
use std::ops::Sub;
use std::str::FromStr;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams::{
    store::StoreAddBigInt, store::StoreSetIfNotExists, store::StoreSetIfNotExistsString,
};

// Curator entities track the cumulative signalled amount, not the current amount
#[substreams::handlers::store]
fn store_cumulative_curator_signalled(events: Events, s: StoreAddBigInt) {
    let signalled_events = events.signalled_events.unwrap();

    for signalled in signalled_events.signalled_events {
        s.add(
            signalled.ordinal,
            utils::generate_key(&signalled.curator),
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
            utils::generate_key(&burned.curator),
            BigInt::from_str(&burned.tokens).unwrap(),
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
fn store_epoch_signal(store_changes: StorageChanges, store: StoreGetBigInt, s: StoreAddBigInt) {
    let curation_pools = store_changes.curation_pools.unwrap();
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::zero() {
                let current_epoch = epoch_count.sub(1).to_string();
                for curation_pool in curation_pools.curation_pools {
                    s.add(
                        curation_pool.ordinal,
                        &current_epoch,
                        BigInt::from_str(&curation_pool.new_signal)
                            .unwrap()
                            .sub(BigInt::from_str(&curation_pool.old_signal).unwrap()),
                    );
                }
            }
        }
        None => (),
    }
}

#[substreams::handlers::store]
fn store_signal_amount(events: Events, s: StoreAddBigInt) {
    let signalled_events = events.signalled_events.unwrap();
    let burned_events = events.burned_events.unwrap();

    for signalled in signalled_events.signalled_events {
        s.add(
            1,
            Hex(&signalled.subgraph_deployment_id).to_string(),
            BigInt::from_str(&signalled.signal).unwrap(),
        );
    }
    for burned in burned_events.burned_events {
        s.add(
            1,
            Hex(&burned.subgraph_deployment_id).to_string(),
            BigInt::from_str(&burned.signal).unwrap().neg(),
        );
    }
}

#[substreams::handlers::store]
fn store_graph_account_curator(events: Events, s: StoreSetIfNotExistsString) {
    let signalled_events = events.signalled_events.unwrap();
    for signalled in signalled_events.signalled_events {
        s.set_if_not_exists(
            signalled.ordinal,
            utils::generate_key(&signalled.curator),
            &utils::generate_key(&signalled.curator),
        );
    }
}
