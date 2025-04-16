use crate::nft::NFTMetadata;
use uuid::Uuid;
use blake3::Hasher;

pub fn hash_metadata_commitment(meta: &NFTMetadata) -> String {
    let mut hasher = Hasher::new();
    hasher.update(meta.name.as_bytes());
    hasher.update(meta.description.as_bytes());
    hasher.update(meta.image_cid.as_bytes());
    hasher.update(meta.attributes.as_bytes());
    hasher.finalize().to_hex().to_string()
}

pub fn generate_nft_id() -> String {
    Uuid::new_v4().to_string()
}
