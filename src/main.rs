use blst::min_pk::SecretKey;
use clap::Parser;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::process;

/// A tool for generating BLS12-381 keys
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Private key in hex format (with or without 0x prefix)
    #[arg(short, long)]
    private_key: Option<String>,
}

/// Generate a new BLS secret key using random bytes
fn generate_secret_key(rng: &mut impl RngCore) -> Result<SecretKey, blst::BLST_ERROR> {
    let mut ikm = [0u8; 32];
    rng.fill_bytes(&mut ikm);
    SecretKey::key_gen(&ikm, &[])
}

/// Parse a hex-encoded private key into a SecretKey
fn parse_private_key(sk_hex: &str) -> Result<SecretKey, String> {
    let sk_hex = if sk_hex.starts_with("0x") {
        &sk_hex[2..]
    } else {
        sk_hex
    };

    let sk_bytes = hex::decode(sk_hex).map_err(|e| format!("Invalid hex format: {}", e))?;
    SecretKey::from_bytes(&sk_bytes).map_err(|e| format!("Invalid private key: {:?}", e))
}

/// Generate a key pair (private and public keys)
fn generate_key_pair(rng: &mut impl RngCore) -> Result<(SecretKey, Vec<u8>), String> {
    let sk =
        generate_secret_key(rng).map_err(|e| format!("Failed to generate secret key: {:?}", e))?;
    let pk = sk.sk_to_pk();
    let pk_bytes = pk.compress();
    Ok((sk, pk_bytes.to_vec()))
}

/// Derive public key from private key
fn derive_public_key(sk: &SecretKey) -> Vec<u8> {
    let pk = sk.sk_to_pk();
    pk.compress().to_vec()
}

fn main() {
    let args = Args::parse();
    let mut rng = StdRng::from_os_rng();

    let (sk, pk_bytes) = match &args.private_key {
        Some(sk_hex) => {
            let sk = parse_private_key(sk_hex).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });
            let pk_bytes = derive_public_key(&sk);
            (sk, pk_bytes)
        }
        None => generate_key_pair(&mut rng).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        }),
    };

    if args.private_key.is_none() {
        println!("Private key: 0x{}", hex::encode(sk.to_bytes()));
    }
    println!("Public key: 0x{}", hex::encode(pk_bytes));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_parse_private_key() {
        // Test with 0x prefix
        let sk_hex = "0x0000000000000000000000000000000000000000000000000000000000000001";
        let result = parse_private_key(sk_hex);
        assert!(result.is_ok());

        // Test without 0x prefix
        let sk_hex = "0000000000000000000000000000000000000000000000000000000000000001";
        let result = parse_private_key(sk_hex);
        assert!(result.is_ok());

        // Test invalid hex
        let sk_hex = "0xinvalid";
        let result = parse_private_key(sk_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_key_pair() {
        // Use a deterministic RNG for testing
        let mut rng = StepRng::new(0, 1);
        let result = generate_key_pair(&mut rng);
        assert!(result.is_ok());

        let (sk, pk_bytes) = result.unwrap();
        assert_eq!(sk.to_bytes().len(), 32); // Private key is 32 bytes
        assert_eq!(pk_bytes.len(), 48); // Public key is 48 bytes
    }

    #[test]
    fn test_derive_public_key() {
        // Create a test secret key
        let mut rng = StepRng::new(0, 1);
        let sk = generate_secret_key(&mut rng).unwrap();

        let pk_bytes = derive_public_key(&sk);
        assert_eq!(pk_bytes.len(), 48); // Public key is 48 bytes
    }
}
