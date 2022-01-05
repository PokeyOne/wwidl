/*
    What Was I Doing Last?, this is a command line utility that makes writing
    notes on what you were doing last easier.

    Copyright (C) 2022  Mateo Carreras

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

    File created by Mateo Carreras January 4, 2022
 */

mod config;
mod commands;

use std::path::PathBuf;
use config::Config;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "wwidl", version, author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Check for the latest notes in the current directory
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Check {
        /// The directory to check
        #[clap(required = false, parse(from_os_str), default_value = ".")]
        path: PathBuf,
        /// Show all the notes for the current directory
        #[clap(long, short)]
        all: bool,
    },
    /// Add a note to the current directory or a specified directory
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Note {
        /// The directory to check
        #[clap(required = false, parse(from_os_str), default_value = ".")]
        path: PathBuf,
        /// The note to add
        #[clap(required = true, long, short)]
        note: String
    },
}

fn main() {
    let args = Cli::parse();
    let config = Config::load();

    match &args.command {
        Command::Check { path, all } => {
            commands::check::execute(path, config, *all);
        }
        Command::Note { path, note } => {
            commands::note::execute(path, config, Some(note.clone()));
        }
    }
}