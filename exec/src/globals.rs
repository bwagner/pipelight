// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Error Handling
use log::trace;
use miette::{IntoDiagnostic, Result};

use std::env;

/**
Lazy global that contains the default shell to be used by the invoked processes.
*/
pub static SHELL: Lazy<Arc<Mutex<String>>> = Lazy::new(|| Arc::new(Mutex::new("sh".to_owned())));

/**
Lazy global that contains the default output directory to be used by the invoked processes
that pipe their outputs(stdout/stderr) into files.
*/
pub static OUTDIR: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(".pipelight/_internals/out".to_owned())));

/**
Returns the  user session shell when found.
*/
pub fn get_shell() -> Result<()> {
    trace!("Get shell");
    let user_shell = env::var("SHELL");
    match user_shell {
        Ok(res) => {
            *SHELL.lock().unwrap() = res.to_owned();
        }
        Err(_) => {
            (*SHELL.lock().unwrap()).to_owned();
        }
    }
    Ok(())
}
