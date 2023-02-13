mod abi;
mod pb;
mod db;
use pb::erc20;
use substreams::prelude::*;
use std::str::FromStr;
use substreams::{log, hex, store::StoreAddBigInt,store::StoreSetProto, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::store;
use substreams::store::{DeltaBigInt};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};
use substreams::scalar::BigInt;
use substreams_ethereum::Event;

// Contract Addresses

substreams_ethereum::init!();

// -------------------- INITIAL MAPS --------------------


// -------------------- STORES --------------------



// -------------------- MAPS FOR ENTITY CHANGES --------------------
// We have an entity change map for each entity in our subgraph schema.graphql 
// These maps take necessary stores or maps as inputs and create/update corresponding entitites in the subgraph using entity changes


// -------------------- GRAPH_OUT --------------------
// Final map for executing all entity change maps together 
// Run this map to check the health of the entire substream 

#[substreams::handlers::map]
pub fn graph_out(
    ) -> Result<EntityChanges, substreams::errors::Error> {

}

// -------------------- KEY GENERATORS --------------------

