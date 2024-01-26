// Structs
use cli::types::{Commands, PreCommands};
// Error Handling
use miette::Result;
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
            Commands::PreCommands(pre_commands) => match pre_commands {
                PreCommands::Init(_) => {
                    pre_commands.start()?;
                    // Deprecated config polling
                    //
                    // set_globals()?;
                    // Set watcher
                    // let args = CLI.lock().unwrap().clone();
                    // if Config::get()?.has_watchable()? {
                    // Service::new(Action::Watch, Some(args))?.should_detach()?;
                    // } else {
                    // watch::Watcher::kill()?;
                    // }
                    // Set git hooks
                    // if Config::get()?.has_git_flag()? {
                    // Hook::enable()?;
                    // } else {
                    // Hook::disable()?;
                    // }
                }
                _ => pre_commands.start()?,
            },
            Commands::PostCommands(post_commands) => {
                set_globals()?;
                post_commands.start()?;
            }
        };
        Ok(())
    }
}
