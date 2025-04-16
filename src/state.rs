// Basic state management (stub for example)
use std::collections::HashMap;

pub struct NFTState {
    pub nfts: HashMap<String, NFT>,
}

#[derive(Clone)]
pub struct NFT {
    pub id: String,
    pub owner: String,
    pub metadata_commitment: String,
    pub royalty_percentage: Option<u8>,
    pub staking_locked: bool,
}

impl NFTState {
    pub fn new() -> Self {
        NFTState {
            nfts: HashMap::new(),
        }
    }
    
    pub fn add_nft(&mut self, nft: NFT) {
        self.nfts.insert(nft.id.clone(), nft);
    }
}
