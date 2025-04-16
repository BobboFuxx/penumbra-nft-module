use crate::types::NFT;

pub fn stake_nft(nft: &mut NFT) {
    nft.staking_locked = true;
    println!("NFT {} has been staked.", nft.id);
}

pub fn unstake_nft(nft: &mut NFT) {
    nft.staking_locked = false;
    println!("NFT {} has been unstaked.", nft.id);
}
