use wasm_bindgen::prelude::*;

use bee_message::prelude::{Address, Ed25519Address};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::{Chain, Curve, Seed},
    },
};

use core::convert::TryInto;
extern crate console_error_panic_hook;
use wasm_bindgen::JsValue;

/// Convert errors so they are readable in JS
pub fn err<T>(error: T) -> JsValue
where
    T: ToString,
{
    error.to_string().into()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn generate_mnemonic() -> Result<String, JsValue> {
    let mut entropy = [0u8; 32];
    getrandom::getrandom(&mut entropy).map_err(err)?;
    // utils::rand::fill(&mut entropy).map_err(err)?;
    let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| format!("{:?}", e))
        .map_err(err)?;
    Ok(mnemonic)
}

#[wasm_bindgen]
pub fn generate_address(mnemonic: &str) -> Result<JsValue, JsValue> {
    let mut result = Vec::new();
    result.push(format!("Mnemonic: {}", mnemonic));
    let mut mnemonic_seed = [0u8; 64];
    mnemonic_to_seed(mnemonic, &"", &mut mnemonic_seed);
    result.push(
    "mnemonic_to_seed(mnemonic, &\"\", &mut mnemonic_seed) https://github.com/iotaledger/crypto.rs/blob/317a138a263621c1861613dee9531e958f652c6e/src/keys/bip39.rs#L17"
        .to_string(),
    );
    result.push(format!("Seed bytes: {:?}", mnemonic_seed));
    result.push(format!("Seed hex encoded: {}", hex::encode(mnemonic_seed)));
    let seed = Seed::from_bytes(&mnemonic_seed);

    let master_key = seed.to_master_key(Curve::Ed25519);
    result.push(
        "seed.to_master_key(Curve::Ed25519) https://github.com/iotaledger/crypto.rs/blob/8b2177e0dab66af8b5511f0477e9dd1cc3a1a94d/src/keys/slip10.rs#L48"
        .to_string(),
    );
    result.push(format!("Master key bytes encoded: {:?}", master_key));

    result.push(
            "seed.to_master_key(Curve::Ed25519) https://github.com/iotaledger/crypto.rs/blob/8b2177e0dab66af8b5511f0477e9dd1cc3a1a94d/src/keys/slip10.rs#L48"
            .to_string(),
        );
    result.push(format!(
        "Master key secret_key bytes encoded: {:?}",
        master_key.secret_key().map_err(err)?.to_le_bytes()
    ));
    result.push(format!(
        "Master key secret_key hex encoded: {}",
        hex::encode(master_key.secret_key().map_err(err)?.to_le_bytes())
    ));

    let account_index = 0;
    let internal = false;
    let address_index = 0;

    // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    let chain_vec = vec![44, 4218, account_index, internal as u32, address_index];
    result.push(format!("Bip32 chain: {:?}", chain_vec));
    let chain = Chain::from_u32_hardened(chain_vec);
    result.push(
            "Chain::from_u32_hardened(chain_vec) https://github.com/iotaledger/crypto.rs/blob/8b2177e0dab66af8b5511f0477e9dd1cc3a1a94d/src/keys/slip10.rs#L152"
            .to_string(),
    );

    result.push(
            "seed.derive(Curve::Ed25519, &chain) https://github.com/iotaledger/crypto.rs/blob/8b2177e0dab66af8b5511f0477e9dd1cc3a1a94d/src/keys/slip10.rs#L54"
            .to_string(),
    );
    let derived_key = seed.derive(Curve::Ed25519, &chain).map_err(err)?;
    result.push(format!("Derived key bytes encoded: {:?}", derived_key));
    result.push(format!(
        "Derived key secret key bytes encoded: {:?}",
        derived_key.secret_key().map_err(err)?.to_le_bytes()
    ));
    result.push(format!(
        "Derived key secret key hex encoded: {}",
        hex::encode(derived_key.secret_key().map_err(err)?.to_le_bytes())
    ));

    result.push(
            "seed.derive(Curve::Ed25519, &chain) https://github.com/iotaledger/crypto.rs/blob/8b2177e0dab66af8b5511f0477e9dd1cc3a1a94d/src/keys/slip10.rs#L54"
            .to_string(),
    );
    let public_key = derived_key
        .secret_key()
        .map_err(err)?
        .public_key()
        .to_compressed_bytes();

    result.push(format!(
        "Derived key public key bytes encoded: {:?}",
        public_key
    ));
    result.push(format!(
        "Derived key public key hex encoded: {}",
        hex::encode(public_key)
    ));

    result.push("let address = Blake2b256::digest(&public_key)".to_string());
    // Hash the public key to get the address
    let address = Blake2b256::digest(&public_key).try_into().map_err(err)?;
    let ed25519_address = Ed25519Address::new(address);
    result.push(format!("Address bytes {:?}", ed25519_address.as_ref()));
    result.push(format!("Address hex encoded {}", ed25519_address));
    let bee_address = Address::Ed25519(ed25519_address);

    result.push(format!(
        "Address Bech32 encoded {}",
        bee_address.to_bech32("iota")
    ));
    JsValue::from_serde(&result).map_err(err)
}
