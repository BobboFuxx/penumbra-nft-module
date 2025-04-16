use crate::types::NFT;

pub fn stake_nft(nft: &mut NFT) {
    nft.staking_locked = true;
}

pub fn unstake_nft(nft: &mut NFT) {
    nft.staking_locked = false;
}
