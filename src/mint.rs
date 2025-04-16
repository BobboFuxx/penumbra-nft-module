use crate::types::{NFTMetadata, NFT};
use crate::utils::{hash_metadata_commitment, generate_nft_id};

pub fn mint_nft(owner: String, metadata: NFTMetadata) -> NFT {
    let commitment = hash_metadata_commitment(&metadata);
    let id = generate_nft_id();

    NFT {
        id,
        owner,
        metadata_commitment: commitment,
        royalty_percentage: None,
        staking_locked: false,
    }
}
