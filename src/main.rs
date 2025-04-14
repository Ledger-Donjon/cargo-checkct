// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::PathBuf, time::Duration};

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};

mod add;
mod common;
mod init;
mod run;

use add::add_driver;
use init::init_workspace;
use run::run_binsec;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        /// Set the path to the directory in which to place the checkct workspace,
        /// if it is different from the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Sets the name of the first constant-time verification driver to be created
        /// in the newly created chechct workspace. Defaults to "driver" if not set.
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,
    },
    Run {
        /// Set the path to the target workspace (containing the checkct directory),
        /// if it is not in the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Set a timeout in seconds. If not set, defaults to 10 minutes (600 seconds).
        #[arg(short, long, value_name = "SECONDS")]
        timeout: Option<u64>,
    },
    Add {
        /// Set the path to the checkct workspace,
        /// if it is different from the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Sets the name of the constant-time verification driver to be created.
        #[arg(short, long, value_name = "NAME")]
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { dir, name } => {
            let dir = dir.unwrap_or(std::env::current_dir()?);
            let name = &name.unwrap_or("driver".to_owned());
            init_workspace(&dir, name)
        }
        Command::Run { dir, timeout } => {
            let dir = dir.unwrap_or(std::env::current_dir()?).join("checkct");
            let timeout = timeout.unwrap_or(600);
            match run_binsec(&dir, Duration::from_secs(timeout))? {
                run::Status::Secure => {
                    println!("SECURE");
                    Ok(())
                }
                run::Status::Insecure => {
                    println!("INSECURE");
                    bail!("Insecure code!")
                }
                run::Status::Unknown => {
                    println!("UNKNOWN");
                    Ok(())
                }
            }
        }
        Command::Add { dir, name } => {
            let dir = dir.unwrap_or(std::env::current_dir()?);
            add_driver(&dir, &name)
        }
    }
}
