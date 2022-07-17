use super::parser;

#[derive(Clone, PartialEq)]
pub enum Strategy {
    Contains,
    Startswith,
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
        _ => panic!("Invalid strategy"),
    };

    AppConfig {
        pattern: args.pattern,
        strategy,
        casesensitive: args.casesensitive,
        contract: args.contract,
        threads: args.threads as u32,
    }
}
