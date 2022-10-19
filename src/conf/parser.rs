use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The pattern to look for
    #[clap(short, long, value_parser)]
    pub pattern: String,

    /// "contains", "startswith" or "trailing"
    #[clap(short, long, value_parser)]
    pub strategy: String,

    /// Whether the pattern is case sensitive
    #[clap(short, long, value_parser, default_value_t = false)]
    pub casesensitive: bool,

    /// Search for a contract address
    #[clap(long, value_parser, default_value_t = false)]
    pub contract: bool,

    /// Number of threads to use
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub threads: u8,

    /// continuous mode
    #[clap(long, value_parser, default_value_t = false)]
    pub continuous: bool,

    /// Calculate the deployment address using create2, must set bytecode and deployer address.
    #[clap(long, value_parser, default_value_t = false)]
    pub create2: bool,

    /// Bytecode of the contract for create2
    #[clap(long, value_parser, default_value = "")]
    pub bytecode: String,

    /// Deployer address for create2
    #[clap(long, value_parser, default_value = "")]
    pub deployer: String,
}

pub(crate) fn parse() -> Args {
    Args::parse()
}
