use anyhow::anyhow;
use clap::Parser;
use tree_sitter_stack_graphs::cli::provided_languages::Subcommands;
use tree_sitter_stack_graphs::NoCancellation;

fn main() -> anyhow::Result<()> {
    let lc = match tree_sitter_stack_graphs_java::try_language_configuration(&NoCancellation) {
        Ok(lc) => lc,
        Err(err) => {
            eprintln!("{}", err);
            return Err(anyhow!("Language configuration error"));
        }
    };
    let cli = Cli::parse();
    cli.subcommand.run(vec![lc])
}

#[derive(Parser)]
#[clap(about, version)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommands,
}
