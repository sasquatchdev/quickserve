use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The hostname which the server will run on.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// The port which the server will run on.
    #[arg(short, long, default_value = "3040")]
    pub port: u16,

    /// The location of the directory the server will serve.
    #[arg(short, long, default_value = ".")]
    pub root: String,

    /// The location of the "fallback" file the server will use as 404.
    #[arg(long, default_value = "./index.html")]
    pub index: String,
}
