use crate::types::NFT;

pub fn airdrop(nft: &NFT, recipients: Vec<String>) -> Vec<NFT> {
    recipients.into_iter().map(|addr| {
        let mut copy = nft.clone();
        copy.owner = addr;
        copy
    }).collect()
}
