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
    return format!(
        "{}:{}",
        who,
        Hex(id).to_string()
    );
}

pub fn generate_key_indexing_rewards(who: String, id: String) -> String {
    return format!(
        "{}:{}",
        who,
        id,
    );
}

pub fn concat(first: String, second: String) -> String {
    return format!(
        "{}:{}",
        first,
        second,
    );
}
