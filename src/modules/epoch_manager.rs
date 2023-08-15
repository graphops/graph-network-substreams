use crate::pb::erc20::*;
use crate::utils;
use std::ops::Sub;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::{store::StoreAddBigInt, store::StoreSetIfNotExists};
#[substreams::handlers::store]
fn store_epoch_length(events: Events, s: StoreSetString) {
    let epoch_length_updated_events = events.epoch_length_updated_events.unwrap();

    for event in epoch_length_updated_events.epoch_length_updated_events {
        s.set(
            event.ordinal,
            "epoch",
            &utils::concat(event.last_length_update_block, event.epoch_length),
        );
    }
}

#[substreams::handlers::store]
fn store_epoch_count(store: StoreGetString, clock: Clock, s: StoreAddBigInt) {
    match store.get_last("epoch") {
        Some(value) => {
            let last_updated_block = value.as_str().split(":").nth(0).unwrap().to_string();
            let epoch_length = value.as_str().split(":").last().unwrap().to_string();
            // right now this is uint64 but it should be changed to BigInt when SF implements it
            if (clock.number - last_updated_block.parse::<u64>().unwrap())
                % epoch_length.parse::<u64>().unwrap()
                == 0
            {
                s.add(1, "epoch", BigInt::one())
            }
        }
        None => (),
    }
}

#[substreams::handlers::store]
fn store_epoch_start(store: StoreGetBigInt, clock: Clock, s: StoreSetIfNotExistsBigInt) {
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::zero() {
                let current_epoch = epoch_count.sub(1).to_string();
                s.set_if_not_exists(1, current_epoch, &clock.number.into())
            }
        }
        None => (),
    }
}

#[substreams::handlers::store]
fn store_epoch_end(store: StoreGetBigInt, clock: Clock, s: StoreSetIfNotExistsBigInt) {
    match store.get_last("epoch") {
        Some(epoch_count) => {
            if epoch_count > BigInt::one() {
                let previous_epoch = epoch_count.clone().sub(2).to_string();
                s.set_if_not_exists(1, previous_epoch, &clock.number.into());
            }
        }
        None => (),
    }
}
