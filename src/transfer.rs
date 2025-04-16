use crate::types::NFT;

pub fn transfer_nft(nft: &mut NFT, new_owner: String) {
    nft.owner = new_owner;
}
