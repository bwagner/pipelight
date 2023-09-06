// Error Handling
use miette::{Error, Result};

// Logger
use log::info;
use utils::logger::logger;

// Colors
use crate::display::set_override;

// Pipeline types
use crate::{Config, Getters, Logs, Pipeline};

// Clap - command line lib
use clap::{builder::PossibleValue, Args, Command, FromArgMatches, ValueEnum, ValueHint};
// use std::str::FromStr;

// Cli core types
use crate::cli::interface::{Cli, ColoredOutput, Commands, LogsCommands, WatchCommands};

// Cli core functions
use crate::cli::actions::print;
use crate::cli::actions::prompt;
use crate::cli::actions::run;
use crate::cli::actions::stop;
use crate::cli::actions::trigger;
use crate::cli::actions::watch;

use crate::globals::{set_globals, CLI, PORTAL};
use clap_complete::shells::Shell;

impl Cli {
    /// Build and Launch the cli
    pub fn launch() -> Result<()> {
        set_globals();

        let args;
        unsafe {
            args = *CLI;
        };

        // Set verbosity level
        let verbosity = args.verbose.log_level_filter();
        logger.lock().unwrap().level(&verbosity);

        // Set internal verbosity level
        let verbosity = args.internal_verbose.log_level_filter();
        logger.lock().unwrap().internal_level(&verbosity);

        match args.commands {
            Commands::Ls(list) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                // Launch watcher
                if Config::get()?.has_watch_flag().is_ok() {
                    watch::create_watcher()?;
                } else {
                    watch::destroy_watcher()?;
                }
                if list.name.is_some() {
                    let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                    print::inspect(&pipeline, list.json)?;
                } else {
                    print::list()?;
                }
            }
            Commands::Inspect(list) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                // info!("Listing piplines");
                if list.name.is_some() {
                    let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                    print::inspect(&pipeline, list.json)?;
                } else {
                    prompt::inspect_prompt()?;
                }
            }
            Commands::Watch(watch) => {
                Config::new(args.config.clone(), args.raw.clone())?;
                match watch.commands {
                    // Set global config
                    None => {
                        info!("Watching for changes");
                        watch::launch(args.attach)?;
                    }
                    Some(watch_cmd) => match watch_cmd {
                        WatchCommands::Kill => {
                            watch::destroy_watcher()?;
                        }
                    },
                }
            }
            Commands::Trigger(trigger) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                info!("Triggering pipelines");
                trigger::launch(args.attach, trigger.flag)?;
            }
            Commands::Run(pipeline) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                if pipeline.name.is_some() {
                    info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                    run::launch(pipeline.name.unwrap(), args.attach, pipeline.trigger.flag)?;
                } else {
                    prompt::run_prompt(args.attach, pipeline.trigger.flag)?;
                }
            }
            Commands::Stop(pipeline) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                info!(
                    "Stopping pipeline {:#?} with every attached and detached subprocess",
                    pipeline.name
                );
                if pipeline.name.is_some() {
                    stop::stop(&pipeline.name.unwrap())?;
                }
            }
            Commands::Completion(shell) => {
                let shell = Shell::from_str(&shell.name, true);
                if shell.is_ok() {
                    Cli::print_completion(shell.unwrap())?;
                } else {
                    return Err(Error::msg("Couldn't determine shell"));
                }
            }
            Commands::Init(_) => {
                // create file
            }
            Commands::Logs(logs) => {
                Config::new(args.config.clone(), args.raw.clone())?;
                // Set colors
                if logs.display.color.is_some() {
                    match ColoredOutput::from(&logs.display.color.unwrap()) {
                        ColoredOutput::Always => set_override(true),
                        ColoredOutput::Never => set_override(false),
                    }
                }

                match logs.commands {
                    None => {
                        let mut pipelines;
                        if logs.display.name.is_some() {
                            pipelines = Logs::get_many_by_name(&logs.display.name.unwrap())?;
                        } else {
                            pipelines = Logs::get()?;
                        }
                        if logs.display.json {
                            print::json(&pipelines)?;
                        } else {
                            print::pretty(&mut pipelines)?;
                        }
                    }
                    Some(logs_cmd) => match logs_cmd {
                        LogsCommands::Rm => {
                            logger.lock().unwrap().clear()?;
                        }
                    },
                };
            }
        }
        Ok(())
    }
}