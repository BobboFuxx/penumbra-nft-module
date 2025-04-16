use std::collections::HashMap;
use crate::types::NFT;

#[derive(Default)]
pub struct NFTState {
    pub nfts: HashMap<String, NFT>,
}
