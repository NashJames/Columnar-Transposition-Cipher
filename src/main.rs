mod transposition;

use crate::transposition::transposition;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "Columnar Transposition Cipher")]
struct Opts {
    /// Activates decryption mode
    #[clap(short, long)]
    decrypt: bool,
    /// The message to be encrypted/decrypted
    msg: String,
    /// The keyword(s) used to encrypt/decrypt
    key: String,
}

/// Collects the program arguments and sends them to the cipher
fn main() {
    let opts: Opts = Opts::parse();

    transposition(opts.decrypt, &opts.msg, &opts.key);
}
