#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_valid_address_check() {
        let res = utils::is_possible_pattern("00");
        assert!(res);
        let res = utils::is_possible_pattern("abcdef0123456789");
        assert!(res);
        let res = utils::is_possible_pattern("cvb");
        assert!(!res);
        let res = utils::is_possible_pattern("!");
        assert!(!res);
        let res = utils::is_possible_pattern("0x");
        assert!(!res);
    }

    #[test]
    fn test_calculate_difficulty() {
        let res = utils::calculate_difficulty("asd", false);
        assert_eq!(res, 16u64.pow(3));
    }
    #[test]
    fn test_calculate_estimated_time() {
        let res = utils::calculate_estimated_time(50, 100);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_calculate_estimated_time_zero_divison() {
        let res = utils::calculate_estimated_time(0, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_calculate_time_left() {
        let res = utils::time_left(90, 10);
        assert_eq!(res, 80);
        let res = utils::time_left(90, 91);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_generate_eth_address() {
        let wallet = eth::Wallet::new();
        let (private, public) = (&wallet.private_key, &wallet.public_key);
        assert_eq!(private.len(), 64);
        assert_eq!(public.len(), 40);
        assert!(!public.starts_with("0x"));
    }

    #[test]
    fn test_generate_contract_address() {
        let wallet = eth::Wallet {
            private_key: "".to_string(),
            public_key: "Ff4a7BE855756B282c211eEfb5e1c5b0c71abDF4".to_string(),
        };
        let contract_address = eth::generate_contract_address(&wallet);

        println!(
            "{} {} {}",
            &wallet.private_key, &wallet.public_key, contract_address
        );

        let wallet = eth::Wallet {
            private_key: "".to_string(),
            public_key: "a6E4dC8578C80ba05E531dB19aA88cd3Bc7F76A5".to_string(),
        };
        let contract_address = eth::generate_contract_address(&wallet);

        assert_eq!(
            eth::checksum(&contract_address),
            "3DD2613dFCEc27f8AcaDe4AcBc30fF4B9737435D"
        );
    }

    #[test]
    fn test_checksum() {
        let addr_lowercase = "e0fc04fa2d34a66b779fd5cee748268032a146c0";
        let checksummed = eth::checksum(addr_lowercase);
        assert_eq!(checksummed, "e0FC04FA2d34a66B779fd5CEe748268032a146c0");

        let addr_uppercase = "E0FC04FA2D34A66B779FD5CEE748268032A146C0";
        let checksummed = eth::checksum(addr_uppercase);
        assert_eq!(checksummed, "e0FC04FA2d34a66B779fd5CEe748268032a146c0");
    }
}
