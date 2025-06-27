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
    /// List of supported commands used for autocompletion.
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
        // slice input text up to current cursor position
        let input = &line[..pos];
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return Ok((0, Vec::new()));
        }

        // complete first token for commands
        if tokens.len() == 1 && !line.ends_with(' ') {
            let prefix = tokens[0];
            let candidates = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            return Ok((0, candidates));
        }

        // provide autocompletion for file/directory arguments in 'load' command
        if tokens[0] == "load"
            && tokens.len() >= 2
            && matches!(tokens[1], "-f" | "--file" | "-d" | "--dir")
            && (tokens.len() == 3 || (tokens.len() == 2 && line.ends_with(' ')))
        {
            return self.file_completer.complete(line, pos, ctx);
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
        if !self.commands.iter().any(|c| c.eq(token)) {
            return Cow::Borrowed(line);
        }

        // bold and color the token for display
        let styled_token = token
            .bold()
            .with(Color::Green)  // change to preference
            .to_string();

        let leading = &line[..first_non_space];
        let trailing = &line[end_of_token..];
        Cow::Owned(format!("{}{}{}", leading, styled_token, trailing))
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        // customize this style as desired
        let colored = prompt.with(Color::Blue).bold().to_string();
        Cow::Owned(colored)
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
