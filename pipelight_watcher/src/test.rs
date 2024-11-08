// Error handling
use miette::{IntoDiagnostic, Result};
use std::env;
// Globals
use std::sync::Arc;

use crate::build::*;
use crate::*;

use std::future::Future;
use watchexec::{action::ActionHandler, Config, Watchexec};
use watchexec_signals::Signal;

/**
* The watcher main action.
*
* Modify the triggering env by setting the action to watch
* And try to trigger pipelines.
*/
pub fn watch_and_trigger() -> Result<()> {
    Ok(())
}

/**
* Wrap the main action into a common signal handling action handler.
*/
pub fn default_action_handler(
    mut action: ActionHandler,
) -> Box<dyn Future<Output = ActionHandler> + Send + Sync> {
    // Pipeline execution
    watch_and_trigger().unwrap();

    // Handle Stop signals
    if action
        .signals()
        .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
    {
        action.quit();
    }

    // Actions
    return Box::new(async { action });
}
impl Watcher {
    /**
     * Build an appropriate watcher that:
     * - self reconfigures on ignore file changes
     * - ignores pipelight autogenerated tmp files
     * - can trigger pipelines
     */
    pub async fn build() -> Result<Watchexec> {
        // Create a Watchexec with action handler
        let watchexec = Watchexec::default();
        // let watchexec = Watchexec::new_async(default_action_handler)?;
        watchexec.config.on_action_async(default_action_handler);

        Ok(watchexec)
    }
    pub async fn get_config() -> Result<Arc<Config>> {
        let watchexec = Watchexec::default();
        Ok(watchexec.config)
    }
    pub async fn set_action(
        &self,
        handler: impl (Fn(ActionHandler) -> Box<dyn Future<Output = ActionHandler> + Send + Sync>)
            + Send
            + Sync
            + 'static,
    ) -> Result<Arc<Config>> {
        self.config.on_action_async(handler);
        Ok(self.config.clone())
    }
    pub async fn set_filters(&self) -> Result<Self> {
        // Search for an ignore file to set a watch filter
        match Self::get_ignore_path() {
            Ok(res) => {
                let filterer = Self::make_filter_configuration(&res).await?;
                self.config.filterer(Arc::new(filterer));
            }
            Err(_) => {
                let filterer = Self::make_default_filter_configuration()?;
                self.config.filterer(Arc::new(filterer));
            }
        }
        // Watch only the current directory
        self.config.pathset(vec![env::current_dir().unwrap()]);
        Ok(self.to_owned())
    }
}

/**
Build and launch the custom watcher
*/
#[tokio::main]
pub async fn launch() -> Result<()> {
    // Kill already running watcher
    Watcher::kill_homologous()?;
    let watchexec = Watcher::build().await?;
    watchexec.main().await.into_diagnostic()??;
    Ok(())
}

#[cfg(test)]
mod watcher {
    // Env
    use super::*;
    use crate::build::*;
    use std::env;
    use std::fs;
    use std::fs::remove_dir_all;
    use std::{thread, time};

    // Globals
    use crate::Watcher;
    use pipelight_exec::Process;
    use pipelight_utils::teleport::Portal;
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    // Logger
    use log::warn;
    // Fancy color
    use colored::Colorize;
    // Process finder
    use pipelight_exec::Finder;

    fn print_cwd() -> Result<()> {
        let path = env::current_dir().into_diagnostic()?;
        let string = path.to_str().unwrap();
        println!("$pwd is {}", string.blue());
        Ok(())
    }

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Build watcher
        let res = Watcher::build().await;
        assert!(res.is_ok());
        Ok(())
    }

    async fn try_start() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Watcher::start()?;
        let watchexec = Watcher::build().await?;
        watchexec.main().await.into_diagnostic()??;
        Ok(())
    }

    /**
    Try to retrieve an ignore file
    */
    #[test]
    fn test_utils() -> Result<()> {
        let res = Watcher::get_ignore_path()?;
        println!("{}", res);
        Ok(())
    }

    fn run_watcher(dir: &str) -> Result<()> {
        fs::create_dir_all(dir).into_diagnostic()?;
        print_cwd()?;

        env::set_current_dir(dir).into_diagnostic()?;
        print_cwd()?;

        // Run watcher
        let mut process = Process::new()
            .stdin("cargo run --bin pipelight init --template toml")
            .to_owned();
        process.run().into_diagnostic()?;
        let mut process = Process::new()
            .stdin("cargo run --bin pipelight watch --attach")
            .to_owned();
        process.background().detach().run().into_diagnostic()?;
        // let res = Watcher::has_homologous_already_running()?;

        Ok(())
    }

    #[test]
    pub fn test_single_watcher() -> Result<()> {
        let root = env::current_dir().into_diagnostic()?;
        let root = root.to_str().unwrap();

        // Create tmp dir
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        for dir in vec![a.clone(), a] {
            run_watcher(&dir)?;
            env::set_current_dir(&root).into_diagnostic()?;
        }

        env::set_current_dir(&test_dir).into_diagnostic()?;

        // Wait for propagation
        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);

        let finder = Watcher::find_all()?;
        println!("{:#?}", finder);

        // Clean
        // Bug or feature?
        // The action of removing directories that are watched stops the watcher.
        // env::set_current_dir(root).into_diagnostic()?;
        // remove_dir_all(test_dir).into_diagnostic()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 1);

        finder.kill()?;
        Ok(())
    }

    /**
    Run watchers in unrelated projects
    */
    // #[test]
    pub fn test_multiple_watcher() -> Result<()> {
        let root = env::current_dir().into_diagnostic()?;
        let root = root.to_str().unwrap();

        // Create tmp dir to run isolated watchers
        let test_dir = "./test_dir_tmp/watcher".to_owned();
        let a = test_dir.clone() + "/a";
        let b = test_dir.clone() + "/b";

        for dir in vec![a, b] {
            run_watcher(&dir)?;
            env::set_current_dir(&root).into_diagnostic()?;
        }

        env::set_current_dir(&test_dir).into_diagnostic()?;

        // Wait for propagation
        let throttle = time::Duration::from_millis(1000);
        thread::sleep(throttle);

        let finder = Watcher::find_all()?;
        println!("{:#?}", finder);

        // Clean
        // Bug or feature?
        // The action of removing directories that are watched stops the watcher.
        env::set_current_dir(root).into_diagnostic()?;
        remove_dir_all(test_dir).into_diagnostic()?;

        assert_eq!(finder.clone().matches.unwrap().len(), 2);

        finder.kill()?;
        Ok(())
    }
}
