use crate::state::NFTState;

pub fn stake_nft(state: &mut NFTState, nft_id: &str) -> Result<(), String> {
    if let Some(nft) = state.get_nft_mut(nft_id) {
        nft.staking_locked = true;
        Ok(())
    } else {
        Err("NFT not found".to_string())
    }
}

pub fn unstake_nft(state: &mut NFTState, nft_id: &str) -> Result<(), String> {
    if let Some(nft) = state.get_nft_mut(nft_id) {
        nft.staking_locked = false;
        Ok(())
    } else {
        Err("NFT not found".to_string())
    }
}
