use clap::Parser;

/// Simple P2P Gossiping Application
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Period in seconds for sending messages
    #[clap(short = 't', long, default_value_t = 5)]
    pub period: u64,

    /// Port to bind the peer
    #[clap(short = 'p', long)]
    pub port: u16,

    /// Address of the peer to connect to
    #[clap(short = 'c', long)]
    pub connect: Option<String>,
}

pub fn get_args() -> Args {
    Args::parse()
}
