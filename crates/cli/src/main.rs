use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// Phone Connect: collega il telefono Android al PC sulla rete locale.
#[derive(Parser)]
#[command(name = "phc", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Avvia il daemon in background (connessione, appunti, icona nel tray).
    Daemon,
    /// Mostra il QR code per associare un nuovo telefono.
    Pair,
    /// Mostra lo stato del daemon e dei dispositivi associati.
    Status,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let name = match cli.command {
        Command::Daemon => "daemon",
        Command::Pair => "pair",
        Command::Status => "status",
    };
    eprintln!(
        "phc {name}: non ancora implementato (protocollo v{})",
        phc_core::PROTOCOL_VERSION
    );
    ExitCode::FAILURE
}
