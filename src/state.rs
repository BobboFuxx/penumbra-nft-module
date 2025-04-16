use std::collections::HashMap;
use crate::types::NFT;

pub struct NFTState {
    pub nfts: HashMap<String, NFT>,
}

impl NFTState {
    pub fn new() -> Self {
        Self {
            nfts: HashMap::new(),
        }
    }

    pub fn add_nft(&mut self, nft: NFT) {
        self.nfts.insert(nft.id.clone(), nft);
    }

    pub fn get_nft(&self, id: &str) -> Option<&NFT> {
        self.nfts.get(id)
    }

    pub fn get_nft_mut(&mut self, id: &str) -> Option<&mut NFT> {
        self.nfts.get_mut(id)
    }
}
