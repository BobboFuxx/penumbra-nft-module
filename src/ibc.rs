use crate::types::NFT;

pub fn export_nft_for_ibc(nft: &NFT) -> String {
    // Serialize NFT for IBC transfer to Osmosis or Stargaze
    serde_json::to_string(nft).unwrap()
}

pub fn import_nft_from_ibc(serialized: &str) -> NFT {
    serde_json::from_str(serialized).unwrap()
}
