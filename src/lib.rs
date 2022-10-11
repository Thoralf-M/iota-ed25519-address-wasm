use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{
        bip39::{mnemonic_to_seed, wordlist},
        slip10::{Chain, Curve, Seed},
    },
};
use iota_client::{
    block::address::{Address, AliasAddress, Ed25519Address, NftAddress},
    Client,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use core::convert::TryInto;
extern crate console_error_panic_hook;
use wasm_bindgen::JsValue;

use std::str::FromStr;

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
    let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| format!("{:?}", e))
        .map_err(err)?;
    Ok(mnemonic)
}

#[wasm_bindgen]
pub fn get_node_info(node_url: &str) -> Result<Promise, JsValue> {
    let node_url = node_url.to_owned();
    Ok(wasm_bindgen_futures::future_to_promise(async move {
        let client = Client::builder()
            .with_node(&node_url)
            .map_err(err)?
            .with_node_sync_disabled()
            .finish()
            .map_err(err)?;

        // for testing, async on client fails, reqwest works
        let info = client.hex_to_bech32(&"0x296f8b2f8c4a2c28821ebdcc7e591a20d172058a695d16502432ee99d8f013d0", Some(&"smr".to_string())).map_err(err)?;

        // let info = Client::get_node_info(&node_url, None).await.map_err(err)?;
        // let info = client.get_info().await.map_err(err)?;
        // let client = reqwest::Client::new();
        // let info = client.get(node_url).send().await.map_err(err)?.text().await.map_err(err)?;
        serde_wasm_bindgen::to_value(&info).map_err(err)
    }))
}

#[wasm_bindgen]
pub fn change_bech32_hrp(bech32_address: &str, new_bech32_hrp: &str) -> Result<JsValue, JsValue> {
    let (_bech32_hrp, address) = Address::try_from_bech32(bech32_address).map_err(err)?;
    serde_wasm_bindgen::to_value(&vec![
        serde_json::to_string(&address).map_err(err)?,
        address.to_bech32(new_bech32_hrp),
    ])
    .map_err(err)
}

#[wasm_bindgen]
pub fn to_bech32_address(
    address: &str,
    bech32_hrp: &str,
    address_type: u8,
) -> Result<String, JsValue> {
    let address = match address_type {
        Ed25519Address::KIND => Address::Ed25519(Ed25519Address::from_str(address).map_err(err)?),
        AliasAddress::KIND => Address::Alias(AliasAddress::from_str(address).map_err(err)?),
        NftAddress::KIND => Address::Nft(NftAddress::from_str(address).map_err(err)?),
        _ => return Err(serde_wasm_bindgen::to_value("Invalid address type").map_err(err)?),
    };
    Ok(address.to_bech32(bech32_hrp))
}

#[wasm_bindgen]
pub fn generate_address(
    mnemonic: &str,
    coin_type: u32,
    account_index: u32,
    internal: bool,
    address_index: u32,
    bech32_hrp: &str,
) -> Result<JsValue, JsValue> {
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| format!("Invalid mnemonic: {:?}", e))?;
    let mut result = Vec::new();
    let mut mnemonic_seed = [0u8; 64];
    mnemonic_to_seed(mnemonic.trim(), "", &mut mnemonic_seed);

    let seed = Seed::from_bytes(&mnemonic_seed);

    // 44 is for BIP 44 (HD wallets)
    let chain_vec = vec![44, coin_type, account_index, internal as u32, address_index];
    let chain = Chain::from_u32_hardened(chain_vec);

    let derived_key = seed.derive(Curve::Ed25519, &chain).map_err(err)?;

    let public_key = derived_key.secret_key().public_key().to_bytes();

    // Hash the public key to get the address
    let address = Blake2b256::digest(&public_key).try_into().map_err(err)?;
    let ed25519_address = Ed25519Address::new(address);
    result.push(format!("{}", ed25519_address));
    let bee_address = Address::Ed25519(ed25519_address);

    result.push(bee_address.to_bech32(bech32_hrp));
    serde_wasm_bindgen::to_value(&result).map_err(err)
}

#[wasm_bindgen]
pub fn generate_address_with_logs(
    mnemonic: &str,
    coin_type: u32,
    account_index: u32,
    internal: bool,
    address_index: u32,
    bech32_hrp: &str,
) -> Result<JsValue, JsValue> {
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| format!("Invalid mnemonic: {:?}", e))?;
    let mut result = Vec::new();
    result.push(format!("Mnemonic: {}", mnemonic));
    let mut mnemonic_seed = [0u8; 64];
    mnemonic_to_seed(mnemonic.trim(), "", &mut mnemonic_seed);
    result.push(
    "mnemonic_to_seed(mnemonic, &\"\", &mut mnemonic_seed) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/bip39.rs#L16"
        .to_string(),
    );
    result.push(format!("Seed bytes: {:?}", mnemonic_seed));
    result.push(format!("Seed hex encoded: {}", hex::encode(mnemonic_seed)));
    let seed = Seed::from_bytes(&mnemonic_seed);

    let master_key = seed.to_master_key(Curve::Ed25519);
    result.push(
        "seed.to_master_key(Curve::Ed25519) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/slip10.rs#L46"
        .to_string(),
    );
    result.push(format!("Master key bytes encoded: {:?}", master_key));

    result.push(
            "seed.to_master_key(Curve::Ed25519) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/slip10.rs#L46"
            .to_string(),
        );
    result.push(format!(
        "Master key secret_key bytes encoded: {:?}",
        master_key.secret_key().to_bytes()
    ));
    result.push(format!(
        "Master key secret_key hex encoded: {}",
        hex::encode(master_key.secret_key().to_bytes())
    ));

    // 44 is for BIP 44 (HD wallets)
    let chain_vec = vec![44, coin_type, account_index, internal as u32, address_index];
    result.push(format!("Bip32 chain: {:?}", chain_vec));
    let chain = Chain::from_u32_hardened(chain_vec);
    result.push(
            "Chain::from_u32_hardened(chain_vec) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/slip10.rs#L157"
            .to_string(),
    );

    result.push(
            "seed.derive(Curve::Ed25519, &chain) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/slip10.rs#L52"
            .to_string(),
    );
    let derived_key = seed.derive(Curve::Ed25519, &chain).map_err(err)?;
    result.push(format!("Derived key bytes encoded: {:?}", derived_key));
    result.push(format!(
        "Derived key secret key bytes encoded: {:?}",
        derived_key.secret_key().to_bytes()
    ));
    result.push(format!(
        "Derived key secret key hex encoded: {}",
        hex::encode(derived_key.secret_key().to_bytes())
    ));

    result.push(
            "seed.derive(Curve::Ed25519, &chain) https://github.com/iotaledger/crypto.rs/blob/752321d556ab7f391d7f3afb98b71adeef6d23b3/src/keys/slip10.rs#L52"
            .to_string(),
    );
    let public_key = derived_key.secret_key().public_key().to_bytes();

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
        bee_address.to_bech32(bech32_hrp)
    ));
    serde_wasm_bindgen::to_value(&result).map_err(err)
}
