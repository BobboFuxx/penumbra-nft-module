use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_cid: String,        // IPFS CID or encrypted reference
    pub attributes: String,       // JSON string of attributes
    pub shielded: bool,           // Whether metadata is shielded
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFT {
    pub id: String,               // Unique NFT ID (UUID or ZK commitment hash)
    pub owner: String,            // Shielded address
    pub metadata: NFTMetadata,
    pub staked: bool,
    pub royalties: Option<u64>,   // Optional royalties in %
    pub view_key: Option<String>, // Optional view key
}

#[derive(Default)]
pub struct NFTState {
    pub nfts: Vec<NFT>,
}

impl NFTState {
    pub fn get_nft(&self, id: &str) -> Option<NFT> {
        self.nfts.iter().find(|n| n.id == id).cloned()
    }

    pub fn mint_nft(&mut self, nft: NFT) {
        self.nfts.push(nft);
    }

    pub fn transfer_nft(&mut self, id: &str, new_owner: String) -> Result<(), String> {
        if let Some(nft) = self.nfts.iter_mut().find(|n| n.id == id) {
            nft.owner = new_owner;
            Ok(())
        } else {
            Err("NFT not found".into())
        }
    }

    pub fn stake_nft(&mut self, id: &str) -> Result<(), String> {
        if let Some(nft) = self.nfts.iter_mut().find(|n| n.id == id) {
            nft.staked = true;
            Ok(())
        } else {
            Err("NFT not found".into())
        }
    }

    pub fn unstake_nft(&mut self, id: &str) -> Result<(), String> {
        if let Some(nft) = self.nfts.iter_mut().find(|n| n.id == id) {
            nft.staked = false;
            Ok(())
        } else {
            Err("NFT not found".into())
        }
    }

    pub fn update_view_key(&mut self, id: &str, view_key: String) -> Result<(), String> {
        if let Some(nft) = self.nfts.iter_mut().find(|n| n.id == id) {
            nft.view_key = Some(view_key);
            Ok(())
        } else {
            Err("NFT not found".into())
        }
    }

    pub fn list_nfts_for_owner(&self, owner: &str) -> Vec<NFT> {
        self.nfts
            .iter()
            .filter(|nft| nft.owner == owner)
            .cloned()
            .collect()
    }
}
