use std::borrow::Cow;

use crossterm::style::{Color, Stylize};
use rustyline::{
    CompletionType, Context, Helper, Result,
    completion::{Completer, FilenameCompleter, Pair},
    highlight::{Highlighter, CmdKind},
    hint::Hinter,
    validate::Validator,
};

/// Helper for REPL, implementing completion, highlighting, and validation.
pub struct ReadLineHelper {
    /// Supported commands for autocompletion.
    commands: Vec<String>,
    /// Completer for file paths.
    file_completer: FilenameCompleter,
}

impl ReadLineHelper {
    /// Create a new helper with the given commands.
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            commands,
            file_completer: FilenameCompleter::new(),
        }
    }
}

impl Helper for ReadLineHelper {}

impl Completer for ReadLineHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        // slice input up to cursor pos
        let input = &line[..pos];
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return Ok((0, Vec::new()));
        }

        // complete first token for commands
        if tokens.len() == 1 && !line.ends_with(' ') {
            let prefix = tokens[0].to_ascii_uppercase();
            let candidates = self
                .commands
                .iter()
                .filter(|cmd| cmd.to_ascii_uppercase().starts_with(&prefix))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            return Ok((0, candidates));
        }

        // complete arguments for 'load' command
        let command = tokens[0];
        if command.eq_ignore_ascii_case("load") {
            // determine path prefix
            let start = line
                .find(command)
                .expect("command must be present")
                + command.len()
                + 1;
            let path_prefix = if tokens.len() > 1 {
                &line[start..pos]
            } else {
                ""
            };

            // delegate to filename completer
            let (off, file_candidates) =
                self.file_completer.complete(path_prefix, path_prefix.len(), ctx)?;
            let global_offset = start + off;
            return Ok((global_offset, file_candidates));
        }

        // no suggestions
        Ok((0, Vec::new()))
    }
}

impl Hinter for ReadLineHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None  // no hints
    }
}

impl Validator for ReadLineHelper {}

impl Highlighter for ReadLineHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        // skip empty lines
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            return Cow::Borrowed(line);
        }

        let first_non_space = match line.find(|c: char| !c.is_whitespace()) {
            Some(i) => i,
            None => return Cow::Borrowed(line),
        };
        let rest = &line[first_non_space..];
        let end_rel = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let end_of_token = first_non_space + end_rel;
        let token = &line[first_non_space..end_of_token];

        // only highlight exact command matches
        if !self.commands.iter().any(|c| c.eq_ignore_ascii_case(token)) {
            return Cow::Borrowed(line);
        }

        // uppercase and color the token for display
        let styled_token = token
            .to_ascii_uppercase()
            .with(Color::Green)  // change to preference
            .bold()
            .to_string();

        let leading = &line[..first_non_space];
        let trailing = &line[end_of_token..];
        Cow::Owned(format!("{}{}{}", leading, styled_token, trailing))
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        _completion: CompletionType,
    ) -> Cow<'c, str> {
        Cow::Borrowed(candidate)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        true
    }
}
