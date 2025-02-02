use clap::{  Parser, Subcommand };
use cli_styles::custom_styles;
use super::cli_styles;

#[derive(Debug, Parser)]
#[command(
    version,
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true
)]
#[command(
    name = "Python Manager",
    version = "0.1.0",
    author = "Fulrix",
    about = "A simple CLI to manage Python virtual enviroonments using Astral UV",
    styles = custom_styles()
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        about = "Install Astral UV",
        long_about = "This command installs Astral UV"
    )]
    Install,

    #[command(
        about = "Check Astral UV",
        long_about = "This command checks if Astral UV is installed"
    )]
    Check,

    #[command(
        about = "Uninstall Astral UV",
        long_about = "This command uninstalls Astral UV"
    )]
    Uninstall,
}

