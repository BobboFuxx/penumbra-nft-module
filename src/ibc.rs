use crate::{types::{NFT, NFTMetadata}};
use base64::{engine::general_purpose, Engine};

pub fn export_nft_for_ibc(nft: &NFT) -> String {
    // Serialize NFT metadata for ICS721 (Stargaze)
    let payload = serde_json::json!({
        "name": nft.metadata.name,
        "description": nft.metadata.description,
        "image": format!("ipfs://{}", nft.metadata.image_cid),
        "attributes": nft.metadata.attributes,
        "royalties": nft.royalties.unwrap_or(0),
        "origin": "penumbra"
    });
    general_purpose::STANDARD.encode(payload.to_string())
}

pub fn import_nft_from_ibc(serialized: &str) -> NFT {
    let decoded = general_purpose::STANDARD.decode(serialized).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&decoded).unwrap();

    NFT {
        id: uuid::Uuid::new_v4().to_string(),
        owner: "imported_owner".into(), // can be dynamically set later
        metadata: NFTMetadata {
            name: json["name"].as_str().unwrap().into(),
            description: json["description"].as_str().unwrap().into(),
            image_cid: json["image"].as_str().unwrap().replace("ipfs://", ""),
            attributes: json["attributes"].as_str().unwrap().into(),
            shielded: false, // unshielded post-import
        },
        staked: false,
        royalties: json["royalties"].as_u64(),
        view_key: None,
    }
}
