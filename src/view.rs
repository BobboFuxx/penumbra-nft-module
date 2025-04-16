use crate::types::{NFT, NFTMetadata};

pub fn view_metadata(nft: &NFT, viewing_key: Option<&str>) -> Option<NFTMetadata> {
    if let Some(_key) = viewing_key {
        // Decrypt logic placeholder
        Some(NFTMetadata {
            name: "Example".to_string(),
            description: "An NFT on Penumbra".to_string(),
            image_cid: "bafy...".to_string(),
            attributes: "{}".to_string(),
            shielded: true,
        })
    } else {
        None
    }
}
