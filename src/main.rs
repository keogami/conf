use anyhow::{Context, Result};
use dirs;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use colored::*;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Alias a config path
    Alias {
        alias: String,
        path: PathBuf,
    },

    /// Edit the config for the given alias
    Edit {
        alias: String,
    },

    List,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Aliases {
    aliases: HashMap<String, PathBuf>,
}

fn file_path() -> Result<PathBuf, anyhow::Error> {
    let data_dir = dirs::data_dir().context("Couldn't find out the data directory.")?;
    let conf_dir = &data_dir.join("conf");
    std::fs::create_dir_all(conf_dir).context("Coudldn't create the data directory for conf.")?;

    Ok(data_dir.join("conf/alias.json"))
}

impl Aliases {
    fn read() -> Result<Self, anyhow::Error> {
        let file = File::open(file_path()?).context("Couldn't open the alias file.")?;

        let reader = BufReader::new(file);

        let aliases = serde_json::from_reader(reader)?;

        Ok(aliases)
    }

    fn write(&self) -> Result<(), anyhow::Error> {
        let file = File::create(file_path()?).context("Couldn't create the alias file.")?;

        let writer = BufWriter::new(file);

        let u = serde_json::to_writer(writer, self)?;

        Ok(u)
    }
}

fn create_alias(alias: String, path: PathBuf) -> Result<()> {
    let mut aliases = Aliases::read().unwrap_or_default();

    aliases.aliases.insert(alias, path);

    aliases.write().context("Couldn't write the alias file.")?;

    Ok(())
}

fn edit_alias(alias: String) -> Result<()> {
    let aliases = Aliases::read().context("There aren't any aliases yet.")?;

    let path = aliases
        .aliases
        .get(&alias)
        .context("There's no alias for that.")?;

    let _ = edit::edit_file(path).context("Couldn't open the editor.")?;

    Ok(())
}

fn list_alias() -> Result<()> {
    let aliases = Aliases::read().context("There aren't any aliases yet.")?;

    for (alias, _path) in aliases.aliases {
        println!("{}", alias.green());
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    match args.command {
        Command::Alias { path, alias } => create_alias(alias, path),

        Command::Edit { alias } => edit_alias(alias),

        Command::List => list_alias(),
    }
}
