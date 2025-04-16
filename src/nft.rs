use crate::types::{NFTMetadata, NFT};
use crate::mint::mint_nft;
use crate::utils::{hash_metadata_commitment, generate_nft_id};

pub fn create_nft(owner: String, metadata: NFTMetadata) -> NFT {
    let commitment = hash_metadata_commitment(&metadata);
    let id = generate_nft_id();
    mint_nft(owner, metadata)
}

pub fn update_nft_metadata(nft: &mut NFT, new_metadata: NFTMetadata) {
    let new_commitment = hash_metadata_commitment(&new_metadata);
    nft.metadata_commitment = new_commitment;
}
