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
//! Work in progress. Currently only supports GitHub Actions.

use std::io::Write;

/// Represents a CI/CD provider (GitHub Actions, Azure DevOps, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Provider {
    GitHub,
    None,
}
    
impl Provider {
    /// Detects the CI/CD provider from the environment variables.
    fn detect() -> Self {
        let is_github = match std::env::var("GITHUB_ACTIONS") {
            Ok(v) => v.eq_ignore_ascii_case("true"),
            Err(_) => false,
        };

        if is_github {
            Provider::GitHub
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
            // Lock stdout to avoid interleaving with concurrent output
            // Leading newline for column 0, same as start marker
            // Swallow all errors to stay unwind-safe (no panics in Drop)
            let mut stdout = std::io::stdout().lock();
            let _ = writeln!(stdout, "\n::group::{title}");
            let _ = stdout.flush(); // Forces the write to be visible
        }

        Group { provider }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        if self.provider.is_active() {
            let mut stdout = std::io::stdout().lock();
            let _ = writeln!(stdout, "\n::endgroup::");
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
    fn is_active_works() {
        assert!(Provider::GitHub.is_active());
        assert!(!Provider::None.is_active());
    }

}