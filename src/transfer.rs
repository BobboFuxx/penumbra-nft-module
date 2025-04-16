use crate::state::NFTState;

pub fn transfer_nft(state: &mut NFTState, nft_id: &str, new_owner: &str) -> Result<(), String> {
    if let Some(nft) = state.get_nft_mut(nft_id) {
        if nft.staking_locked {
            return Err("NFT is locked in staking".to_string());
        }
        nft.owner = new_owner.to_string();
        Ok(())
    } else {
        Err("NFT not found".to_string())
    }
}
