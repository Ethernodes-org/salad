// Rust’s standard library provides a lot of useful functionality, but assumes support for various
// features of its host system: threads, networking, heap allocation, and others. SGX environments
// do not have these features, so we tell Rust that we don’t want to use the standard library
#![no_std]
#![allow(unused_attributes)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
// The eng_wasm crate allows to use the Enigma runtime, which provides:
//     - Read from state      read_state!(key)
//     - Write to state       write_state!(key => value)
//     - Print                eprint!(...)
extern crate eng_wasm;

// The eng_wasm_derive crate provides the following
//     - Functions exposed by the contract that may be called from the Enigma network
//     - Ability to call functions of ethereum contracts from ESC
extern crate eng_wasm_derive;

// Serialization stuff
extern crate rustc_hex;

// Crypto stuff
extern crate recrypt;

// eng_wasm
use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use eng_wasm_derive::eth_contract;
use eng_wasm::{String, H256, H160, Vec};
use rustc_hex::ToHex;
use recrypt::prelude::*;
use recrypt::Revealed;

// Mixer contract abi
#[eth_contract("IMixer.json")]
struct EthContract;

// State key name "mixer_eth_addr" holding eth address of Mixer contract
static MIXER_ETH_ADDR: &str = "mixer_eth_addr";

// For contract-exposed functions, declare such functions under the following public trait:
#[pub_interface]
pub trait ContractInterface {
    fn construct(mixer_eth_addr: H160);
    fn execute_deal(deal_id: H256, enc_recipients: Vec<u8>) -> String;
}

// The implementation of the exported ESC functions should be defined in the trait implementation
// for a new struct.
// #[no_mangle] modifier is required before each function to turn off Rust's name mangling, so that
// it is easier to link to. Sets the symbol for this item to its identifier.
pub struct Contract;

// Private functions accessible only by the secret contract
impl Contract {
    // Read voting address of VotingETH contract
    fn get_mixer_eth_addr() -> String {
        // TODO: Try unwrap_or
        read_state!(MIXER_ETH_ADDR).unwrap_or_default()
    }

    fn decrypt(message: Vec<u8>) -> Vec<u8> {
        // create a new recrypt
        eprint!("Instantiating Recrypt...");
        let mut recrypt = Recrypt::new();

        // generate a plaintext to encrypt
//        let pt = recrypt.gen_plaintext();

        // generate a public/private keypair and some signing keys
//        let (priv_key, pub_key) = recrypt.generate_key_pair().unwrap();
//        let signing_keypair = recrypt.generate_ed25519_key_pair();

        // encrypt!
//        let encrypted_val = recrypt.encrypt(&pt, &pub_key, &signing_keypair).unwrap();

        // decrypt!
//        let decrypted_val = recrypt.decrypt(encrypted_val, &priv_key).unwrap();
//        decrypted_val.bytes().to_vec()
        eprint!("Got Recrypt");
        message
    }
}

impl ContractInterface for Contract {
    // Constructor function that takes in VotingETH ethereum contract address
    #[no_mangle]
    fn construct(mixer_eth_addr: H160) {
        let mixer_eth_addr_str: String = mixer_eth_addr.to_hex();
        write_state!(MIXER_ETH_ADDR => mixer_eth_addr_str);
    }

    #[no_mangle]
    fn execute_deal(deal_id: H256, enc_recipients: Vec<u8>) -> String {
        eprint!("In execute_deal({:?}, {:?})", deal_id, enc_recipients);
        let result = Self::decrypt(enc_recipients);
        eprint!("The encrypted string: {}", result.to_hex::<String>());

        let encrypted_addresses: Vec<String> = Vec::new();
        eprint!("Mixing address for deal {:?}: {:?}", deal_id, encrypted_addresses);
        let mixer_eth_addr: String = Self::get_mixer_eth_addr();
        let eth_contract = EthContract::new(&mixer_eth_addr);
        let mut recipients: Vec<H160> = Vec::new();
        let address_bytes: [u8; 20] = [0; 20];
        recipients.push(H160::from(&address_bytes));
        let deal_id_uint = U256::from(deal_id);
        eth_contract.distribute(deal_id_uint, recipients);
        "Hello".to_string()
    }
}