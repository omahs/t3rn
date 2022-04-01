use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_finality_grandpa::{AuthorityId, AuthoritySignature};
use sp_runtime::traits::Header as HeaderT;
use sp_runtime::{generic, traits::BlakeTwo256};
use sp_std::prelude::*;

// default MFV settings
pub type BlockNumber = u32;
pub type Hasher = BlakeTwo256;
pub type Header = generic::Header<BlockNumber, Hasher>;

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, Clone, PartialEq, Eq)]
pub struct GrandpaJustification<Header: HeaderT> {
    pub round: u64,
    pub commit:
        finality_grandpa::Commit<Header::Hash, Header::Number, AuthoritySignature, AuthorityId>,
    pub votes_ancestries: Vec<Header>,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let payload = std::fs::read_to_string(&args[2])
        .expect(&format!("usage: {} /tmp/justification_file", args[0]));
    let hex = if payload.trim().starts_with("0x") {
        &payload.trim()[2..]
    } else {
        &payload.trim()
    };
    let scale_encoded = hex::decode(hex).expect("Hex decoding error");
    match args[1].as_str() {
        "blocknumber" => println!("{:?}", block_number(scale_encoded)),
        "authority_set" => println!("{:?}", authority_set(scale_encoded)),
        _ => println!("Unknown subcommand"),
    }
}

fn block_number(scale_encoded: Vec<u8>) -> u32 {
    let justification =
        GrandpaJustification::<Header>::decode(&mut &*scale_encoded).expect("SCALE decoding error");
    justification.commit.target_number
}

fn authority_set(scale_encoded: Vec<u8>) -> String {
    let justification =
        GrandpaJustification::<Header>::decode(&mut &*scale_encoded).expect("SCALE decoding error");
    let mut authorities = vec![];
    for signed in &justification.commit.precommits {
        authorities.push(signed.id.encode());
    }
    serde_json::to_string(&authorities).expect("JSON stringify error")
}
