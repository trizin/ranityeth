use rand::RngCore;
use tiny_keccak::{Hasher, Keccak};

pub fn generate_salt() -> [u8; 32] {
    // Generate a random salt
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_salt(salt: [u8; 32]) -> [u8; 32] {
    // derive salt from the previous one
    let mut derived_salt = salt;
    let mut rng = rand::thread_rng();
    let index = rng.next_u32() as usize % 32;
    let value = rng.next_u32() as u8;
    derived_salt[index] = value;
    derived_salt.rotate_left(1);
    derived_salt
}

pub fn bytecode_keccak(bytecode: &str) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];
    let _h = hex::decode(bytecode).unwrap();
    keccak.update(&_h);
    keccak.finalize(&mut hash);
    hash
}

pub fn calc_addr(address: &str, salt: [u8; 32], bytecode_hash: [u8; 32]) -> String {
    let address = hex::decode(address).unwrap();
    // calculate address
    let mut buf = [0; 85];
    buf[0] = 0xFF;
    buf[1..21].copy_from_slice(&address);
    buf[21..53].copy_from_slice(&salt);
    buf[53..85].copy_from_slice(&bytecode_hash);

    let mut sha3 = Keccak::v256();
    sha3.update(&buf);
    let mut fin = [0; 32];
    sha3.finalize(&mut fin);
    let mut calculated_addr = [0; 20];

    calculated_addr.copy_from_slice(&fin[12..32]);

    // convert to hash
    let hash = hex::encode(calculated_addr);
    return hash;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_addr() {
        let addr = "f426cE76A4925a4AA5Afb4051443D100d33aab33";
        let mut salt_byte = [0; 32];
        let salt = "afe78640665423b7d1bc1ec9ad8f6c16b40a8330afd52489c99e150aeed11dc8";
        let _salt_byte = hex::decode(salt).unwrap();
        salt_byte.copy_from_slice(&_salt_byte);
        let bytecode = "c0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffeec0ffee";
        let bytecode_hash = bytecode_keccak(bytecode);
        let addr = calc_addr(addr, salt_byte, bytecode_hash);
        assert_eq!(addr, "bd9d0e337c453599a99a4565d4d888865a902cbd");
    }

    #[test]
    fn test_generate_random_salt() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn test_derive_salt() {
        let salt = generate_salt();
        let derived_salt = derive_salt(salt);
        assert_ne!(salt, derived_salt);
    }
}
