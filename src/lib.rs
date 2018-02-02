// the following feature flag is used for one tiny thing, and we can get rid of it if we need to
// however i'm already on nightly, this is getting stabilized soonish, so let's just :shipit:
#![feature(nll)]

extern crate comrak;
#[macro_use]
extern crate configure;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate handlebars;
extern crate notify;
extern crate rls_analysis as analysis;
extern crate rls_data as analysis_data;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate simple_server;
#[macro_use]
extern crate slog;
extern crate toml_edit;
extern crate walkdir;

mod cargo;
mod config;
mod error;
pub mod examples;
mod git;
pub mod ops;

pub use config::Config;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

/// this removes the first space from each line of its input.
///
/// this is useful because doc comments geneerally look like this:
///
/// ```text
/// /// some words
/// ```
///
/// see that space before `some`? it's technically part of the comment,
/// so the RLS will give it to us in the docs.
pub fn strip_leading_space(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    // s.len() is going to be long enough, as we're only dropping some characters,
    // so let's preallocate even though it's going to be slightly bigger
    let mut s = s.lines()
        .fold(String::with_capacity(s.len()), |mut s, line| {
            // some lines don't have any content, and so we should handle that.
            if !line.is_empty() {
                s += &line[1..];
            }

            // we still want to retain the newlines in the output, but `lines` strips them
            s += "\n";

            s
        });

    // remove the trailing newline
    s.truncate(s.len() - 1);

    s
}

#[cfg(test)]
mod tests {
    use super::strip_leading_space;

    #[test]
    fn strips_whitespace() {
        let input = " a doc comment
 this is the form they're generally in";

        let result = strip_leading_space(input);

        assert_eq!(
            result,
            "a doc comment
this is the form they're generally in"
        );
    }

    #[test]
    fn works_on_the_empty_string() {
        let result = strip_leading_space("");

        assert_eq!(result, "");
    }

    #[test]
    fn doesnt_panic_on_blank_lines() {
        let input = " Examples of rendering

 This module and its submodules are purely for show. Check out each type of
 item and see how Doxidize decides to render them!";

        let result = strip_leading_space(input);

        assert_eq!(
            result,
            "Examples of rendering

This module and its submodules are purely for show. Check out each type of
item and see how Doxidize decides to render them!"
        );
    }
}
