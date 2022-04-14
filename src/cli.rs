use clap::Parser;

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(short, long, default_value = "dyd.toml")]
    pub manifest: std::path::PathBuf,
}

impl CLI {
    pub fn new() -> Self {
        CLI::parse()
    }
}
