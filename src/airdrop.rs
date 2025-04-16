use crate::{state::NFTState, transfer::transfer_nft};

pub fn airdrop_nft(
    state: &mut NFTState,
    nft_id: &str,
    recipients: Vec<String>,
) -> Result<(), String> {
    for recipient in recipients {
        transfer_nft(state, nft_id, &recipient)?;
    }
    Ok(())
}
