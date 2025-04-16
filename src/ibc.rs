use crate::types::NFT;

pub fn prepare_ibc_packet(nft: &NFT) -> Vec<u8> {
    // Placeholder serialization
    bincode::serialize(nft).unwrap()
}
