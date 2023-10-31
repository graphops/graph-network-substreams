use crate::pb::erc20::*;
use crate::utils;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store::StoreAddBigInt;
use substreams::Hex;

#[substreams::handlers::store]
fn store_version_count(events: Events, s: StoreAddBigInt) {
    let subgraph_version_updated_events = events.subgraph_version_updated_events.unwrap();
    let subgraph_published1_events = events.subgraph_published1_events.unwrap();
    let subgraph_published2_events = events.subgraph_published2_events.unwrap();
    for subgraph_published1 in subgraph_published1_events.subgraph_published1_events {
        s.add(
            subgraph_published1.ordinal,
            utils::get_subgraph_id(
                &Hex(&subgraph_published1.graph_account).to_string(),
                &subgraph_published1.subgraph_number,
            ),
            BigInt::one(),
        );
    }
    for subgraph_published2 in subgraph_published2_events.subgraph_published2_events {
        s.add(
            subgraph_published2.ordinal,
            &subgraph_published2.subgraph_id,
            BigInt::one(),
        );
    }
    for subgraph_version_updated in subgraph_version_updated_events.subgraph_version_updated_events
    {
        s.add(
            subgraph_version_updated.ordinal,
            &subgraph_version_updated.subgraph_id,
            BigInt::one(),
        );
    }
}
#[substreams::handlers::store]
fn store_version(events: Events, version_count: StoreGetBigInt, clock: Clock, s: StoreSetString) {
    let subgraph_version_updated_events = events.subgraph_version_updated_events.unwrap();
    let subgraph_published1_events = events.subgraph_published1_events.unwrap();
    let subgraph_published2_events = events.subgraph_published2_events.unwrap();
    for subgraph_published1 in subgraph_published1_events.subgraph_published1_events {
        let subgraph_id = utils::get_subgraph_id(
            &Hex(&subgraph_published1.graph_account).to_string(),
            &subgraph_published1.subgraph_number,
        );
        let version_count = version_count.get_at(subgraph_published1.ordinal, &subgraph_id);
        s.set(
            subgraph_published1.ordinal,
            utils::generate_version_id(
                subgraph_id,
                Hex(&subgraph_published1.subgraph_deployment_id).to_string(),
                (&version_count.unwrap()).into(),
            ),
            &clock.timestamp.clone().unwrap().to_string(),
        );
    }
    for subgraph_published2 in subgraph_published2_events.subgraph_published2_events {
        let version_count = version_count.get_at(
            subgraph_published2.ordinal,
            &subgraph_published2.subgraph_id,
        );
        s.set(
            subgraph_published2.ordinal,
            utils::generate_version_id(
                Hex(&subgraph_published2.subgraph_id).to_string(),
                Hex(&subgraph_published2.subgraph_deployment_id).to_string(),
                (&version_count.unwrap()).into(),
            ),
            &clock.timestamp.clone().unwrap().to_string(),
        );
    }
    for subgraph_version_updated in subgraph_version_updated_events.subgraph_version_updated_events
    {
        let version_count = version_count.get_at(
            subgraph_version_updated.ordinal,
            &subgraph_version_updated.subgraph_id,
        );
        s.set(
            subgraph_version_updated.ordinal,
            utils::generate_version_id(
                Hex(&subgraph_version_updated.subgraph_id).to_string(),
                Hex(&subgraph_version_updated.subgraph_deployment_id).to_string(),
                (&version_count.unwrap()).into(),
            ),
            &clock.timestamp.clone().unwrap().to_string(),
        );
    }
}
