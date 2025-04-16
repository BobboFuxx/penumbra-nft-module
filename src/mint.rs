use crate::types::{NFTMetadata, NFT};
use crate::utils::{hash_metadata_commitment, generate_nft_id};
use snarkjs::{groth16, Circuit, FieldElement};

pub fn generate_metadata_commitment_proof(metadata: &NFTMetadata) -> String {
    let name_hash = hash_metadata_commitment(&metadata.name);
    let desc_hash = hash_metadata_commitment(&metadata.description);
    let image_cid_hash = hash_metadata_commitment(&metadata.image_cid);
    let attr_hash = hash_metadata_commitment(&metadata.attributes);

    let proof = groth16::generate_proof(
        &Circuit::load("metadata_commitment.circom"), 
        &[FieldElement::from(name_hash), FieldElement::from(desc_hash), FieldElement::from(image_cid_hash), FieldElement::from(attr_hash)]
    ).unwrap();

    proof.to_hex()
}

pub fn mint_nft(owner: String, metadata: NFTMetadata) -> NFT {
    let commitment = hash_metadata_commitment(&metadata);
    let proof = generate_metadata_commitment_proof(&metadata);
    let id = generate_nft_id();

    NFT {
        id,
        owner,
        metadata_commitment: commitment,
        royalty_percentage: None,
        staking_locked: false,
    }
}
