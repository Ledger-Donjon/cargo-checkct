// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::PathBuf, thread, time::Duration};

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};

mod add;
mod common;
mod init;
mod run;

use add::add_driver;
use init::init_workspace;
use run::{Check, Options, Status, overall_status, run_binsec, summarize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a checkct workspace, with a first verification driver
    Init {
        /// Set the path to the library in which to place the checkct workspace,
        /// if it is different from the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Sets the name of the first constant-time verification driver to be created
        /// in the newly created checkct workspace. Defaults to "driver" if not set.
        #[arg(short, long, value_name = "NAME")]
        name: Option<String>,
    },
    /// Build the verification drivers, and verify them with binsec
    Run {
        /// Set the path to the target workspace (containing the checkct directory),
        /// if it is not in the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Set a timeout in seconds, for the verification of each entrypoint.
        #[arg(short, long, value_name = "SECONDS", default_value_t = 600)]
        timeout: u64,

        /// Do not raise an error if binsec cannot conclude on the tested implementation.
        #[arg(long, action)]
        skip_unknown: bool,

        /// Constant-time checks to perform.
        #[arg(
            long,
            value_name = "CHECKS",
            value_delimiter = ',',
            default_value = "control-flow,memory-access"
        )]
        checks: Vec<Check>,

        /// Keep exploring after the first leak of each entrypoint, to report all the leaky
        /// instructions. This can take much longer.
        #[arg(long, action)]
        all_leaks: bool,

        /// Number of binsec analyses to run in parallel. Defaults to the number of CPUs.
        #[arg(short, long, value_name = "N")]
        jobs: Option<usize>,
    },
    /// Add a verification driver to an existing checkct workspace
    Add {
        /// Set the path to the library containing the checkct workspace,
        /// if it is different from the working directory.
        #[arg(short, long, value_name = "PATH")]
        dir: Option<PathBuf>,

        /// Sets the name of the constant-time verification driver to be created.
        #[arg(short, long, value_name = "NAME")]
        name: String,
    },
}

fn main() -> Result<()> {
    // When invoked as `cargo checkct`, cargo passes "checkct" as the first argument
    let mut args = std::env::args_os().collect::<Vec<_>>();
    if args.get(1).is_some_and(|arg| arg == "checkct") {
        args.remove(1);
    }
    let cli = Cli::parse_from(args);

    match cli.command {
        Command::Init { dir, name } => {
            let dir = dir.unwrap_or(std::env::current_dir()?);
            let name = &name.unwrap_or("driver".to_owned());
            init_workspace(&dir, name)
        }
        Command::Run {
            dir,
            timeout,
            skip_unknown,
            checks,
            all_leaks,
            jobs,
        } => {
            let dir = dir.unwrap_or(std::env::current_dir()?).join("checkct");
            let options = Options {
                timeout: Duration::from_secs(timeout),
                checks,
                all_leaks,
                jobs: jobs.unwrap_or_else(|| {
                    thread::available_parallelism().map_or(1, |jobs| jobs.get())
                }),
            };
            let reports = run_binsec(&dir, &options)?;
            summarize(&reports)?;
            match overall_status(&reports) {
                Status::Secure => {
                    println!("SECURE");
                    Ok(())
                }
                Status::Insecure => {
                    println!("INSECURE");
                    bail!("Insecure code!")
                }
                Status::Error => {
                    println!("ERROR");
                    bail!("Some drivers could not be verified!")
                }
                Status::Unknown => {
                    println!("UNKNOWN");
                    if skip_unknown {
                        Ok(())
                    } else {
                        bail!("Unknown status!")
                    }
                }
            }
        }
        Command::Add { dir, name } => {
            let dir = dir.unwrap_or(std::env::current_dir()?);
            add_driver(&dir, &name)
        }
    }
}
