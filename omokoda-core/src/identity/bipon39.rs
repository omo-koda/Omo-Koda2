use bipon39::{entropy_to_mnemonic, mnemonic_to_seed, split_mnemonic};

pub struct Bipon39;

impl Bipon39 {
    pub fn entropy_to_mnemonic(entropy: &[u8]) -> String {
        entropy_to_mnemonic(entropy)
            .map(|words| words.join(" "))
            .unwrap_or_default()
    }

    pub fn mnemonic_to_indices(mnemonic: &str) -> Result<Vec<u8>, String> {
        let words = split_mnemonic(mnemonic);
        let mut indices = Vec::new();
        for word in words {
            let entry = bipon39::entry_by_encoding(word)
                .map_err(|e| format!("Invalid word in mnemonic: {}: {}", word, e))?;
            indices.push(entry.array_index as u8);
        }
        Ok(indices)
    }

    pub fn mnemonic_to_seed(
        mnemonic: &str,
        _agent_id: &str,
        _birth_timestamp: u64,
        _chain_id: &str,
    ) -> [u8; 32] {
        let words = split_mnemonic(mnemonic);
        let seed64 = mnemonic_to_seed(&words, "")
            .expect("BIP39 seed derivation failed")
            .to_vec();
        let mut seed32 = [0u8; 32];
        seed32.copy_from_slice(&seed64[..32]);
        seed32
    }

    pub fn get_odu_index(indices: &[u8]) -> u8 {
        indices.iter().fold(0, |acc, &x| acc ^ x)
    }
}
