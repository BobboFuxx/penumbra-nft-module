// Airdrop logic (this can be extended)
use crate::types::NFT;

pub fn airdrop_nfts(nfts: Vec<NFT>, recipient: String) {
    for nft in nfts {
        println!("Airdropping NFT {} to {}", nft.id, recipient);
    }
}
