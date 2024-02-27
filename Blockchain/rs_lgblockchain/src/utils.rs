use sha2::{Digest, Sha256};

pub const DIFFICULTY_PREFIX: &str = "00";

pub fn hash2binary(hash: &[u8]) -> String {
    let mut ret = String::new();
    hash.iter()
        .for_each(|&c| ret.push_str(format!("{:b}", c).as_str()));
    ret
}

pub fn calculate_hash(
    id: u64,
    timestamp: i64,
    previous_hash: &str,
    data: &str,
    nonce: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    log::info!("mining block ...");
    let mut nonce = 0;

    loop {
        if nonce % 100_000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash2binary(&hash);
        let encoded_hash = hex::encode(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            log::info!(
                "mined! nonce: {}, hash: {}, binary_hash: {}",
                nonce,
                encoded_hash,
                binary_hash
            );
            return (nonce, encoded_hash);
        }
        nonce += 1;
    }
}
