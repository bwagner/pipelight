// Structs
use crate::pipeline::Filters;
use crate::types::{Logs, Pipeline};
// Trait
use crate::traits::Getters;
// Date and Time
use chrono::{DateTime, Local};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};
// Global vars
use crate::globals::LOGS;

impl Logs {
    /**
    Read logs and store them into a global variable.
    Sorted by ascending date by default.
    */
    pub fn hydrate() -> Result<()> {
        // Get global
        let global_logs = LOGS.lock().unwrap().clone();
        if global_logs.is_none() {
            // Read log files
            let json_logs: Vec<String> = cast::Logs::read(".pipelight/logs/")?;
            let mut pipelines: Vec<Pipeline> = vec![];
            for json in json_logs {
                let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
                pipelines.push(pipeline);
            }
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            // Set global
            *LOGS.lock().unwrap() = Some(pipelines);
        }
        Ok(())
    }
}

// Basic getters
impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        Logs::hydrate()?;
        let logs = LOGS.lock().unwrap().clone();
        match logs {
            Some(mut pipelines) => {
                pipelines = Filters::sort_by_date_asc(pipelines)?;
                Ok(pipelines)
            }
            None => Err(Error::msg("Couldn't get a pipeline log from log files")),
        }
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let mut pipelines = Logs::get()?;
        pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        match pipelines.pop() {
            None => {
                let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
                Err(Error::msg(message))
            }
            Some(p) => Ok(p),
        }
    }
}

// More getters
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>> {
        let mut pipelines = Logs::get()?;
        pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        match pipelines.is_empty() {
            false => Ok(pipelines),
            true => {
                let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
                Err(Error::msg(message))
            }
        }
    }
}
