use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct WitsConfig {
    /// Location of Tantivy index.
    #[arg(short, long)]
    pub(crate) tantivy_directory: String,
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Show detailed disk usage.
    DiskUsage {
        /// Restrict to these fields. If empty, all field usage will be displayed.
        fields: Vec<String>,
    },
    /// Show details about fields.
    #[command(subcommand)]
    Fields(FieldsInfo),
}

#[derive(Debug, Subcommand)]
pub(crate) enum FieldsInfo {
    /// List fields available in the index.
    List,
    /// Show information about a field such as top term.
    Show {
        /// Field name
        field: String,
    },
}
