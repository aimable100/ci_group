//! RAII log groups for GitHub Actions.
//!
//! I kept losing CI logs because unclosed groups swallow all output after them.
//! This crate fixes that. Groups close automatically when dropped, even on panic.
//!
//! ```rust,ignore
//! let _g = ci_group::open("Build");
//! build(); // if this panics, the group still closes
//! ```
//!
//! Work in progress. Supports GitHub Actions and Azure Pipelines.

use std::io::Write;

/// Represents a CI/CD provider (GitHub Actions, Azure DevOps, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Provider {
    GitHub,
    Azure,
    None,
}
    
impl Provider {
    /// Detects the CI/CD provider from the environment variables.
    fn detect() -> Self {
        if std::env::var("GITHUB_ACTIONS")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            Provider::GitHub
        } else if std::env::var("TF_BUILD")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            Provider::Azure
        } else {
            Provider::None
        }
    }

    /// Returns true if the provider is active.
    fn is_active(&self) -> bool {
        !matches!(self, Provider::None)
    }

}


/// A collapsible log group. Closes automatically when dropped.
#[must_use = "group closes immediately when dropped. Bind it: let _g = open(...)"]
pub struct Group {
    provider: Provider,
}

impl Group {
    /// Creates a new group with the given title.
    pub fn new(title: &str) -> Self {
        let provider = Provider::detect();

        if provider.is_active() {
            let mut stdout = std::io::stdout().lock();
            let _ = match provider {
                Provider::GitHub => writeln!(stdout, "\n::group::{title}"),
                Provider::Azure => writeln!(stdout, "\n##[group]{title}"),
                Provider::None => Ok(()),
            };
            let _ = stdout.flush();
        }

        Group { provider }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        if self.provider.is_active() {
            let mut stdout = std::io::stdout().lock();
            let _ = match self.provider {
                Provider::GitHub => writeln!(stdout, "\n::endgroup::"),
                Provider::Azure => writeln!(stdout, "\n##[endgroup]"),
                Provider::None => Ok(()),
            };
            let _ = stdout.flush();
        }
    }
}

/// Opens a new log group. Alias for [`Group::new`].
pub fn open(title: &str) -> Group {
    Group::new(title)
}

#[macro_export]
macro_rules! group {
    ($title:expr, $body:block) => {{
        let _guard = $crate::open($title);
        $body
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_github() {
        temp_env::with_var("GITHUB_ACTIONS", Some("true"), || {
            assert_eq!(Provider::detect(), Provider::GitHub);
        });
        temp_env::with_var_unset("GITHUB_ACTIONS", || {
            assert_eq!(Provider::detect(), Provider::None);
        });
    }

    #[test]
    fn detects_github_case_insensitive() {
        temp_env::with_var("GITHUB_ACTIONS", Some("TRUE"), || {
            assert_eq!(Provider::detect(), Provider::GitHub);
        });
        temp_env::with_var("GITHUB_ACTIONS", Some("True"), || {
            assert_eq!(Provider::detect(), Provider::GitHub);
        });
    }

    #[test]
    fn rejects_non_true_values() {
        temp_env::with_var("GITHUB_ACTIONS", Some("false"), || {
            assert_eq!(Provider::detect(), Provider::None);
        });
        temp_env::with_var("GITHUB_ACTIONS", Some(""), || {
            assert_eq!(Provider::detect(), Provider::None);
        });
        temp_env::with_var("GITHUB_ACTIONS", Some("1"), || {
            assert_eq!(Provider::detect(), Provider::None);
        });
    }

    #[test]
    fn detects_azure() {
        temp_env::with_var("TF_BUILD", Some("True"), || {
            assert_eq!(Provider::detect(), Provider::Azure);
        });
        temp_env::with_var_unset("TF_BUILD", || {
            assert_eq!(Provider::detect(), Provider::None);
        });
    }

    #[test]
    fn is_active_works() {
        assert!(Provider::GitHub.is_active());
        assert!(Provider::Azure.is_active());
        assert!(!Provider::None.is_active());
    }

}