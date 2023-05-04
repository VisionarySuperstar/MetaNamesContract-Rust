use sha3::Digest;

/// ## Description
/// Checks provided merkle root validity
/// ## Params
/// * **merkle_root** is an object of type [`str`]
pub fn validate_merkle_root(merkle_root: &str) {
    let mut root_buf: [u8; 32] = [0; 32];
    assert!(
        hex::decode_to_slice(merkle_root, &mut root_buf).is_ok(),
        "Invalid Hex Merkle"
    );
}

/// ## Description
/// Performs a verification of specified merkle proofs
/// ## Params
/// * **merkle_root** is an object of type [`str`]
///
/// * **leaf** is an object of type [`[u8]`]
///
/// * **proof** is an object of type [`[String]`]
pub fn verify_merkle_proof(merkle_root: &str, leaf: &[u8], proof: &[String]) {
    let mut leaf_buf: [u8; 32] = sha3::Keccak256::digest(leaf)
        .as_slice()
        .try_into()
        .expect("Wrong length");

    for p in proof {
        let mut proof_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf).unwrap();

        leaf_buf = if bytes_cmp(leaf_buf, proof_buf) == std::cmp::Ordering::Less {
            sha3::Keccak256::digest(&[leaf_buf, proof_buf].concat())
                .as_slice()
                .try_into()
                .expect("Wrong length")
        } else {
            sha3::Keccak256::digest(&[proof_buf, leaf_buf].concat())
                .as_slice()
                .try_into()
                .expect("Wrong length")
        };
    }

    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(merkle_root, &mut root_buf).unwrap();
    assert!(root_buf == leaf_buf, "Merkle verification failed");
}

fn bytes_cmp(a: [u8; 32], b: [u8; 32]) -> std::cmp::Ordering {
    let mut i = 0;
    while i < 32 {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            _ => i += 1,
        }
    }

    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_merkle_root() {
        let merkle_root = "321db53cd3105ae5f617a265d4154d374c3ce0695bd139e4a5624260789243db";
        validate_merkle_root(merkle_root);
    }

    #[test]
    #[should_panic(expected = "Invalid Hex Merkle")]
    fn test_invalid_merkle_root() {
        let merkle_root = "321db53cd3105ae5f617a265d4154d374c3ce0695bd139e4a5624260789243d";
        validate_merkle_root(merkle_root)
    }

    #[test]
    fn test_varify_merkle_proof() {
        let merkle_root = "321db53cd3105ae5f617a265d4154d374c3ce0695bd139e4a5624260789243db";
        let leaf = "0091c011c7b2d2e41a35b696a20d2dff62105d1aa6";
        let proof =
            ["710c92c04197da66b0229c0b29238c069cf720f52acf4fb2a292cb3df8dc830a".to_string()];

        verify_merkle_proof(merkle_root, leaf.as_bytes(), &proof);
    }

    #[test]
    #[should_panic]
    fn test_verify_invalid_merkle_proof() {
        let merkle_root = "321db53cd3105ae5f617a265d4154d374c3ce0695bd139e4a5624260789243db";
        let leaf = "0091c011c7b2d2e41a35b696a20d2dff62105d1aa6";
        let proof =
            ["43297d829509b8ba92b45435b8888d40c2dbc00691e4a11cdba2977bc8cd18ae".to_string()];

        verify_merkle_proof(merkle_root, leaf.as_bytes(), &proof)
    }

    #[test]
    #[should_panic]
    fn test_verify_invalid_leaf() {
        let merkle_root = "321db53cd3105ae5f617a265d4154d374c3ce0695bd139e4a5624260789243db";
        let leaf = "00665d7b0079304126020b9c89b17bdb159d047440";
        let proof =
            ["710c92c04197da66b0229c0b29238c069cf720f52acf4fb2a292cb3df8dc830a".to_string()];

        verify_merkle_proof(merkle_root, leaf.as_bytes(), &proof)
    }

    #[test]
    #[should_panic]
    fn test_verify_invalid_merkle_root() {
        let merkle_root = "f4187df12859565f27ed06a796fef10009423d6e1b545311373617a7bace5a94";
        let leaf = "0091c011c7b2d2e41a35b696a20d2dff62105d1aa6";
        let proof =
            ["710c92c04197da66b0229c0b29238c069cf720f52acf4fb2a292cb3df8dc830a".to_string()];

        verify_merkle_proof(merkle_root, leaf.as_bytes(), &proof)
    }
}
