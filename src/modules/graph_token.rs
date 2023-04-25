use crate::pb::erc20::*;
use crate::utils;
use std::str::FromStr;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store::StoreAddBigInt;

use substreams_ethereum::NULL_ADDRESS;

// -------------------- STORES --------------------
#[substreams::handlers::store]
fn store_grt_balances(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        s.add(
            transfer.ordinal,
            utils::generate_key(&transfer.from),
            BigInt::from_str(&transfer.value).unwrap().neg(),
        );
        s.add(
            transfer.ordinal,
            utils::generate_key(&transfer.to),
            BigInt::from_str(&transfer.value).unwrap(),
        );
    }
}

#[substreams::handlers::store]
fn store_grt_global(events: Events, s: StoreAddBigInt) {
    let transfers = events.transfers.unwrap();
    for transfer in transfers.transfers {
        if transfer.to == NULL_ADDRESS {
            s.add(
                transfer.ordinal,
                "totalSupply",
                BigInt::from_str(&transfer.value).unwrap().neg(),
            );
            s.add(
                transfer.ordinal,
                "totalGRTBurned",
                BigInt::from_str(&transfer.value).unwrap(),
            );
        }
        if transfer.from == NULL_ADDRESS {
            s.add(
                transfer.ordinal,
                "totalSupply",
                BigInt::from_str(&transfer.value).unwrap(),
            );
            s.add(
                transfer.ordinal,
                "totalGRTMinted",
                BigInt::from_str(&transfer.value).unwrap(),
            );
        }
    }
}
