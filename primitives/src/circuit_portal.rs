use crate::ProofTriePointer;
use snowbridge_core::Verifier;
use sp_std::vec::Vec;

pub trait CircuitPortal<T: frame_system::Config> {
    type EthVerifier: Verifier;

    fn confirm_inclusion(
        gateway_id: [u8; 4],
        _encoded_message: Vec<u8>,
        trie_type: ProofTriePointer,
        maybe_block_hash: Option<Vec<u8>>,
        maybe_proof: Option<Vec<Vec<u8>>>,
    ) -> Result<(), &'static str>;
}
