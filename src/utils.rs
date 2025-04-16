use crate::types::NFTMetadata;
use sha2::{Sha256, Digest};
use uuid::Uuid;

pub fn hash_metadata_commitment(metadata: &NFTMetadata) -> String {
    let mut hasher = Sha256::new();
    hasher.update(metadata.name.as_bytes());
    hasher.update(metadata.description.as_bytes());
    hasher.update(metadata.image_cid.as_bytes());
    hasher.update(metadata.attributes.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn generate_nft_id() -> String {
    Uuid::new_v4().to_string()
}
