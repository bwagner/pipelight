#[cfg(test)]
mod watcher {
    // Globals
    use crate::actions::watch::build::get_ignore_path;
    use crate::actions::watch::{build, Watcher};
    use utils::teleport::Portal;
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    use thiserror::Error;

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Build watcher
        let res = build().await;
        assert!(res.is_ok());
        Ok(())
    }
    // #[tokio::test]
    async fn try_start() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Watcher::start()?;
        let (we, runtime) = build().await?;
        we.main().await.into_diagnostic()?;
        Ok(())
    }

    #[test]
    /**
    Try to retrieve an ignore file
    */
    fn test_utils() -> Result<()> {
        let res = get_ignore_path()?;
        println!("{}", res);
        Ok(())
    }
}
