use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_cid: String,
    pub attributes: String,
    pub shielded: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NFT {
    pub id: String,
    pub owner: String,
    pub metadata_commitment: String,
    pub royalty_percentage: Option<u8>,
    pub staking_locked: bool,
}
