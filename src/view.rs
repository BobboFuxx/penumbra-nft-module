use crate::{state::NFTState, types::NFT};

pub fn reveal_nft(state: &NFTState, id: &str, viewing_key: Option<String>) -> Option<&NFT> {
    // For now, we're not using the viewing_key in logic, but it could be added in zk circuit.
    state.get_nft(id)
}
