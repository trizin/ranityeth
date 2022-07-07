use secp256k1::Secp256k1;
use std::fmt::Write;
use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let (private_key, public_key) = generate_key_address();
        Wallet {
            private_key,
            public_key,
        }
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_contract_address(wallet: &Wallet) -> String {
    let checksummed_address = checksum(&wallet.public_key);

    let bytes = hex::decode(checksummed_address).expect("Unable to unwrap address");
    let nonce: Vec<u8> = vec![0];

    let arr = [&bytes, &nonce];
    let mut encoded = rlp::encode_list::<Vec<u8>, _>(&arr);
    encoded.truncate(encoded.len() - 1);
    encoded.extend(&[128u8]);

    let mut sha3 = Keccak::v256();

    sha3.update(&encoded);

    let mut address: [u8; 32] = [0; 32];
    sha3.finalize(&mut address);

    let mut address_string = String::new();
    for &byte in address.iter().skip(12) {
        write!(&mut address_string, "{:02x}", byte).expect("Unable to write");
    }

    address_string
}

pub fn generate_key_address() -> (String, String) {
    let mut rng = rand::thread_rng();
    let context = Secp256k1::new();

    let (private_key, public_key) = context.generate_keypair(&mut rng);

    let mut private_key_string = String::new();

    for &byte in private_key[..].iter() {
        write!(&mut private_key_string, "{:02x}", byte).expect("Unable to write");
    }

    let mut sha3 = Keccak::v256();
    sha3.update(&public_key.serialize_uncompressed()[1..65]);

    let mut address: [u8; 32] = [0; 32];
    sha3.finalize(&mut address);

    let mut address_string = String::new();
    for &byte in address.iter().skip(12) {
        write!(&mut address_string, "{:02x}", byte).expect("Unable to write");
    }

    (private_key_string, address_string)
}

pub fn checksum(address: &str) -> String {
    let address = address.to_lowercase();

    let address_hash = {
        let mut hasher = Keccak::v256();
        hasher.update(address.as_bytes());
        let mut address: [u8; 32] = [0; 32];
        hasher.finalize(&mut address);

        let mut address_string = String::new();
        for &byte in address.iter() {
            write!(&mut address_string, "{:02x}", byte).expect("Unable to write");
        }

        address_string
    };

    address
        .char_indices()
        .fold("".to_string(), |mut acc, (index, address_char)| {
            // this cannot fail since it's Keccak256 hashed
            let n = u16::from_str_radix(&address_hash[index..index + 1], 16).unwrap();

            if n > 7 {
                // make char uppercase if ith character is 9..f
                acc.push_str(&address_char.to_uppercase().to_string())
            } else {
                // already lowercased
                acc.push(address_char)
            }

            acc
        })
}
