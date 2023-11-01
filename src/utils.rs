use base58::ToBase58;
use bigdecimal::Num;
use hex::FromHex;
use substreams::Hex;
use tiny_keccak::{Hasher, Keccak};

pub fn find_key<T: AsRef<[u8]>>(address: T, slot: u64, order: u32) -> [u8; 32] {
    // Pad the address with leading zeros to make it 32 bytes
    let padded_address = add_padding(address);

    // Convert the number to a byte array
    let padded_slot = add_padding(slot.to_be_bytes());

    // Concatenate the padded address and padded number
    let mut concat = Vec::new();
    concat.extend_from_slice(&padded_address);
    concat.extend_from_slice(&padded_slot);
    let keccak = keccak256(&concat);
    let key = increment_key(keccak, order);
    key
}
fn add_padding<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut padded_input = [0; 32];
    let input_bytes = input.as_ref();

    let input_len = input_bytes.len();
    let pad_len = padded_input.len() - input_len;

    padded_input[pad_len..].copy_from_slice(input_bytes);
    padded_input
}

fn increment_key(mut keccak: [u8; 32], order: u32) -> [u8; 32] {
    let mut carry = order;
    for i in (0..32).rev() {
        let (result, new_carry) = keccak[i].overflowing_add(carry as u8);
        keccak[i] = result;
        carry = new_carry as u32;
        if carry == 0 {
            break;
        }
    }
    keccak
}

pub fn keccak256(data: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut out: [u8; 32] = [0; 32];
    hasher.update(data);
    hasher.finalize(&mut out);
    out
}

// -------------------- KEY GENERATORS --------------------
pub fn generate_key(account: &Vec<u8>) -> String {
    return Hex(account).to_string();
}

pub fn generate_key_delegated_stake(delegator: &Vec<u8>, indexer: &Vec<u8>) -> String {
    return format!(
        "{}:{}",
        Hex(delegator).to_string(),
        Hex(indexer).to_string()
    );
}

pub fn generate_key_query_fee_rebates(who: String, id: &Vec<u8>) -> String {
    return format!("{}:{}", who, Hex(id).to_string());
}

pub fn generate_key_indexing_rewards(who: String, id: String) -> String {
    return format!("{}:{}", who, id,);
}

pub fn concat(first: String, second: String) -> String {
    return format!("{}:{}", first, second,);
}

pub fn generate_ipfs_hash(subgraph_id: String) -> String {
    return Vec::from_hex(format!("{}{}", "1220", subgraph_id))
        .unwrap()
        .to_base58()
        .to_string();
}

pub fn generate_version_id(
    subgraph_id: String,
    subgraph_deployment_id: String,
    version_count: String,
) -> String {
    return concat(concat(subgraph_id, subgraph_deployment_id), version_count);
}

extern crate num_bigint;
extern crate tiny_keccak;

use num_bigint::BigUint;

pub fn get_subgraph_id(graph_account: &str, subgraph_number: &str) -> String {
    // Convert graph account to string and remove '0x' prefix if exists
    let graph_account_str = graph_account.trim_start_matches("0x");

    // Convert subgraph_number (string) to BigUint
    let subgraph_num = BigUint::from_str_radix(subgraph_number.trim_start_matches("0x"), 16)
        .expect("Failed to convert subgraph_number to BigUint");

    // Convert subgraph number to hexadecimal string, ensure it's 64 chars long
    let subgraph_number_str = format!("{:064x}", subgraph_num);

    // Concatenate the strings
    let unhashed_subgraph_id = format!("{}{}", graph_account_str, subgraph_number_str);

    // Hash using Keccak256
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(unhashed_subgraph_id.as_bytes());
    keccak.finalize(&mut output);

    // Convert result to BigUint
    let big_int_representation = BigUint::from_bytes_be(&output);

    format!("{:x}", big_int_representation)
}
