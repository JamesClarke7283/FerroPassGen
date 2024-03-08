use clap::{Parser, Subcommand};
use ferropassgen::{PassphraseGen, PasswordGen, PassGen};
use diceware_wordlists::EFF_LONG_WORDLIST as DICEWARE_WORDLIST;

#[derive(Parser, Debug)]
struct PasswordArgs {
    #[arg(long)]
    length: usize,
    #[arg(long)]
    charset: String,
}

#[derive(Parser, Debug)]
struct PassphraseArgs {
    #[arg(long)]
    length: usize,
    #[arg(long, default_value = "-")]
    separator: char,
    #[arg(long, help = "Use uppercase words")]
    upper: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Password(PasswordArgs),
    Passphrase(PassphraseArgs),
}

#[derive(Parser, Debug)]
#[command(name = "ferropassgen-cli")]
#[command(about = "A command-line tool for generating strong passwords and passphrases", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Password(args) => {
            let password_gen = PasswordGen::new(args.length, args.charset.chars().collect(), None, None).unwrap();
            let password = password_gen.generate();
            println!("Generated password: {password}");
        }
        Commands::Passphrase(args) => {
            let wordlist: Vec<String> = DICEWARE_WORDLIST.iter().map(|&s| s.to_string()).collect();
            let passphrase_gen = PassphraseGen::new(args.length, wordlist, Some(args.separator), Some(args.upper)).unwrap();
            let passphrase = passphrase_gen.generate();
            println!("Generated passphrase: {passphrase}");
        }
    }
}