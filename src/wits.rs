use clap::Parser;
use tantivy::Index;
use tantivy::Result;

use crate::config::{Commands, FieldsInfo, WitsConfig};

mod config;
mod fields;
mod space_usage;

fn index_data(directory: &str) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let args: WitsConfig = WitsConfig::parse();

    index_data(&args.tantivy_directory)?;

    let index = Index::open_in_dir(args.tantivy_directory)?;

    match args.command {
        Commands::Interactive => todo!(),
        Commands::DiskUsage { fields } => space_usage::show_space_usage(&index, fields)?,
        Commands::Fields(field_info) => match field_info {
            FieldsInfo::List => fields::list_field(&index),
            FieldsInfo::Show { field } => todo!(),
        },
    }

    Ok(())
}
