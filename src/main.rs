mod double_cipher;
mod single_cipher;

use crate::double_cipher::double_transposition;
// use crate::single_cipher::single_transposition;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "Double Columnar Transposition Cipher")]
struct Opts {
    #[clap(short, long)]
    decrypt: bool,
    #[clap(short, long)]
    msg: String,
    #[clap(short, long)]
    key: String,
}

/// Collects the program arguments and sends them to the cipher
fn main() {
    let opts: Opts = Opts::parse();

    double_transposition(opts.decrypt, &opts.msg, &opts.key);
    // single_transposition(opts.decrypt, &opts.msg, &opts.key);
}
