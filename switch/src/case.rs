// Colors
use workflow::traits::display::set_override;
// Structs
use clap::ValueEnum;
use clap_complete::shells::Shell;
use cli::types::{
    Cli, ColoredOutput, Commands, DetachableCommands, LogsCommands, PostCommands, PreCommands,
};
use utils::git::Flag;
use workflow::{Config, Getters, Logs, Pipeline, Trigger};
// Template
use templates::Template;
// Error Handling
use log::info;
use miette::{Error, IntoDiagnostic, Result};
// Global vars
use crate::globals::{set_early_globals, set_globals};
use cli::globals::CLI;

pub struct Switch;
impl Switch {
    /// Build and Launch the cli
    pub fn case() -> Result<()> {
        set_early_globals()?;
        let mut args = CLI.lock().unwrap().clone();
        match &mut args.commands {
            Commands::PreCommands(pre_commands) => {
                pre_commands.start()?;
            }
            Commands::PostCommands(post_commands) => {
                set_globals()?;
                post_commands.start()?;
            }
        }
        Ok(())
    }
}