use agy7rust::commands::{Cli, Commands};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Compress { input, output } => {
            agy7rust::commands::compress::run(&input, &output)?;
        }
        Commands::Inspect { input } => {
            agy7rust::commands::inspect::run(&input)?;
        }
        Commands::Verify { input } => {
            agy7rust::commands::verify_cmd::run(&input)?;
        }
        Commands::Replay { input } => {
            agy7rust::commands::replay_cmd::run(&input)?;
        }
        Commands::Adversarial { input } => {
            agy7rust::commands::adversarial::run(&input)?;
        }
        Commands::SchemaCheck { input, schema } => {
            agy7rust::commands::schema_check::run(&input, &schema)?;
        }
        Commands::ContextBuild {
            input,
            schema,
            output,
        } => {
            agy7rust::commands::context_build::run(&input, &schema, &output)?;
        }
        Commands::ContextRender { input, output } => {
            agy7rust::commands::context_render::run(&input, &output)?;
        }
        Commands::ContextValidate { input } => {
            agy7rust::commands::context_validate::run(&input)?;
        }
    }

    Ok(())
}
