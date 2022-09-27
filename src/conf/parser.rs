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
}

pub(crate) fn parse() -> Args {
    Args::parse()
}
