pub fn generate_metadata_commitment_proof(
    name_hash: &str,
    desc_hash: &str,
    image_hash: &str,
    attr_hash: &str,
) -> String {
    format!("poseidon({},{},{},{})", name_hash, desc_hash, image_hash, attr_hash)
}
