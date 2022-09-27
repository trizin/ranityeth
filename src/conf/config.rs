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
    pub threads: u32,
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

    AppConfig {
        pattern: args.pattern,
        strategy,
        casesensitive: args.casesensitive,
        contract: args.contract,
        threads: args.threads as u32,
    }
}
