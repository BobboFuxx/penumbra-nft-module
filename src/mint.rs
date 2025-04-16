use crate::{
    types::{NFT, NFTMetadata},
    utils::{generate_nft_id, hash_metadata_commitment},
    state::NFTState,
};

pub fn mint_nft(
    state: &mut NFTState,
    owner: String,
    metadata: NFTMetadata,
    royalty_percentage: Option<u8>,
) -> String {
    let id = generate_nft_id();
    let commitment = hash_metadata_commitment(&metadata);

    let nft = NFT {
        id: id.clone(),
        owner,
        metadata_commitment: commitment,
        royalty_percentage,
        staking_locked: false,
    };

    state.add_nft(nft);
    id
}
