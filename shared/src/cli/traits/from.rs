use crate::cli::types::{
    Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, Trigger, Watch,
};
use std::fmt;

impl From<&Commands> for String {
    fn from(e: &Commands) -> String {
        let string = format!("{}", e);
        return string;
    }
}
impl From<&Cli> for String {
    fn from(e: &Cli) -> String {
        let string = format!("{}", e.commands);
        return string;
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.name.is_some() {
            string += &self.name.clone().unwrap();
        }
        if self.trigger.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &self.trigger.flag.clone().unwrap();
        }
        if self.trigger.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for DisplayCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.name.is_some() {
            string += &self.name.clone().unwrap();
        }
        if self.json {
            string += " ";
            string += "--json";
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.commands.is_some() {
            match self.commands.clone().unwrap() {
                LogsCommands::Rm => {
                    string += "rm";
                }
            }
            string += &format!("{}", &self.display);
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &self.flag.clone().unwrap();
        }
        if self.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Watch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string;
        match self {
            Commands::Run(pipeline) => {
                string = format!("run {}", pipeline);
            }
            Commands::Raw(raw) => {
                string = "raw".to_owned();
                string += " ";
                string += &raw.string;
            }
            Commands::Stop(pipeline) => {
                string = format!("stop {}", pipeline);
            }
            Commands::Trigger(trigger) => {
                string = format!("trigger {}", trigger);
            }
            Commands::Logs(logs) => {
                string = format!("logs {}", logs);
            }
            Commands::Inspect(_) => string = "inspect".to_owned(),
            Commands::Ls(_) => string = "ls".to_owned(),
            Commands::Watch(_) => string = "watch".to_owned(),
        }
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::types::{
        Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, Trigger,
    };
    use crate::cli::verbosity::Verbosity;
    // Convert cli into string then print
    #[test]
    fn run() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Run(Pipeline {
                name: Some("test".to_owned()),
                trigger: Trigger {
                    attach: false,
                    flag: None,
                },
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(1, 0),
        };
        // print it
        let result = String::from(&cli);
        // println!("{}", result);
        assert_eq!(result, "run test");
    }
    #[test]
    fn logs() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Logs(Logs {
                commands: Some(LogsCommands::Rm),
                display: DisplayCommands {
                    json: false,
                    name: None,
                },
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(1, 0),
        };
        // print it
        let result = String::from(&cli);
        // println!("{}", result);
        assert_eq!(result, "logs rm");
    }
    #[test]
    fn verbosity() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Logs(Logs {
                commands: Some(LogsCommands::Rm),
                display: DisplayCommands {
                    json: false,
                    name: None,
                },
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(1, 0),
        };
        // print it
        let result = String::from(&cli);
        // println!("{}", result);
        assert_eq!(result, "logs rm");
    }
}
