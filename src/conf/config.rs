use crate::utils;

use super::parser;

#[derive(Clone, PartialEq)]
pub enum Strategy {
    Contains,
    Startswith,
    Trailing,
}

#[derive(Clone)]
pub struct AppConfig {
    pub pattern: String,
    pub strategy: Strategy,
    pub casesensitive: bool,
    pub contract: bool,
    pub create2: bool,
    pub threads: u32,
    pub continuous: bool,
    pub deployer: String,
    pub bytecode: String,
}

pub fn get_config() -> AppConfig {
    let args = parser::parse();
    let strategy = match args.strategy.as_str() {
        "contains" => Strategy::Contains,
        "startswith" => Strategy::Startswith,
        "trailing" => Strategy::Trailing,
        _ => panic!("Invalid strategy"),
    };
    if strategy == Strategy::Trailing && args.pattern.len() != 1 {
        panic!("Trailing strategy only accepts a single character pattern");
    }

    if args.contract && args.create2 {
        // deployer cannot be empty
        assert!(
            !args.deployer.is_empty(),
            "Deployer address cannot be empty"
        );

        // bytecode cannot be empty
        assert!(!args.bytecode.is_empty(), "Bytecode cannot be empty");

        // bytecode length must be greater than 32 bytes
        assert!(
            args.bytecode.len() >= 64,
            "Bytecode length must be greater than 32 bytes"
        );

        let _addr = args.deployer.replace("0x", "");
        assert!(
            args.deployer.replace("0x", "").len() == 40,
            "Invalid deployer address"
        );
        assert!(utils::is_possible_pattern(_addr.as_str()));
    }

    if args.continuous && strategy == Strategy::Trailing {
        panic!("Continuous mode is not supported with trailing strategy");
    }

    AppConfig {
        pattern: args.pattern,
        strategy,
        casesensitive: args.casesensitive,
        contract: args.contract,
        create2: args.create2,
        threads: args.threads as u32,
        continuous: args.continuous,
        deployer: args.deployer,
        bytecode: args.bytecode,
    }
}
