use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use substreams::store::{Deltas, DeltaBigInt};
use crate::erc20::{Transfers, Approvals};
use substreams::{ log, Hex};
use substreams::prelude::DeltaProto;
use substreams::scalar::{ BigInt};
use substreams_ethereum::{NULL_ADDRESS};
use std::str::FromStr;
use crate::pb::erc20;
