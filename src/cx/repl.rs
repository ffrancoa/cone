use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

/// Helper structure for the REPL, implementing Completer and other rustyline traits.
pub struct ReadLineHelper {
    /// List of supported commands for autocompleting the first token.
    commands: Vec<String>,
    /// Filename completer for completing file paths after certain commands.
    file_completer: FilenameCompleter,
}

impl ReadLineHelper {
    /// Create a new helper with a list of supported commands.
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            commands,
            file_completer: FilenameCompleter::new(),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////
// Trait implementations
/////////////////////////////////////////////////////////////////////////////////////////

/// Marker trait for helpers.
impl Helper for ReadLineHelper {}

/// Provide completion candidates for commands and arguments.
impl Completer for ReadLineHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        // Text up to the cursor.
        let input = &line[..pos];
        // Split into tokens by whitespace.
        let tokens: Vec<_> = input.split_whitespace().collect();

        // If nothing has been typed, return no suggestions.
        if tokens.is_empty() {
            return Ok((0, Vec::new()));
        }

        // Case 1: completing the first token (command), when there's no trailing space.
        if tokens.len() == 1 && !line.ends_with(' ') {
            let prefix = tokens[0].to_ascii_uppercase();
            let mut candidates = Vec::new();
            for cmd in &self.commands {
                if cmd.to_ascii_uppercase().starts_with(&prefix) {
                    candidates.push(Pair {
                        display: cmd.clone(),
                        replacement: cmd.clone(),
                    });
                }
            }
            // Replace from the start of the token (position 0).
            return Ok((0, candidates));
        }

        // Case 2: completing arguments for specific commands
        let command = tokens[0];
        if command.eq_ignore_ascii_case("load") {
            // Determine the path prefix the user has started typing
            let path_prefix = if tokens.len() > 1 {
                // Everything after "load " up to the cursor
                let start = line
                    .find(command)
                    .expect("command must be present")
                    + command.len()
                    + 1;
                &line[start..pos]
            } else {
                "" // space after command but nothing else
            };

            // Delegate to the filename completer
            let (start_offset, file_candidates) =
                self.file_completer.complete(path_prefix, path_prefix.len(), ctx)?;
            // Adjust the returned start index to the full-line offset
            let global_offset = line
                .find(command)
                .expect("command must be present")
                + command.len()
                + 1
                + start_offset;
            return Ok((global_offset, file_candidates));
        }

        // Default: no suggestions
        Ok((0, Vec::new()))
    }
}

/// Optional: provide inline hints while typing.
impl Hinter for ReadLineHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None // No hints for now
    }
}

/// Empty implementations for highlighting and validation.
impl Highlighter for ReadLineHelper {}
impl Validator for ReadLineHelper {}
