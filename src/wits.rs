use clap::Parser;
use tantivy::{Index, Result};

use crate::config::{Commands, FieldsInfo, WitsConfig};
use crate::fields::Fields;

mod config;
mod fields;
mod space_usage;

fn main() -> Result<()> {
    let args: WitsConfig = WitsConfig::parse();

    let index = Index::open_in_dir(args.tantivy_directory)?;

    match args.command {
        Commands::DiskUsage { fields } => {
            let usage = space_usage::DiskUsage::new(&index, Some(fields))?;
            println!("{}", usage);
        }
        Commands::Fields(field_info) => match field_info {
            FieldsInfo::List => println!("{}", Fields::from(&index)),
            FieldsInfo::Show { field } => fields::detailed_field(&index, field),
        },
    }

    Ok(())
}
