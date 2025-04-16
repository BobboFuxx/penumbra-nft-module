use crate::types::NFT;

pub fn transfer_nft(nft: &mut NFT, new_owner: String, nullifier: Option<String>) {
    if let Some(n) = nullifier {
        store_nullifier(&n);
    }

    nft.owner = new_owner;
}

fn store_nullifier(nullifier: &str) {
    // Store nullifier to track that this NFT has been used
    println!("Storing nullifier: {}", nullifier);
}
