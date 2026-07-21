//! `fast-glob` is a high-performance glob matching crate for Rust, originally forked from [`devongovett/glob-match`](https://github.com/devongovett/glob-match).
//! This crate provides efficient glob pattern matching with support for multi-pattern matching and brace expansion.
//!
//! ## Key Features
//!
//! - Up to 60% performance improvement.
//! - Support for more complex and efficient brace expansion.
//! - Fixed matching issues with wildcard and globstar [`glob-match/issues#9`](https://github.com/devongovett/glob-match/issues/9).
//!
//! ## Examples
//!
//! ```rust
//! use fast_glob::glob_match;
//!
//! let glob = "some/**/n*d[k-m]e?txt";
//! let path = "some/a/bigger/path/to/the/crazy/needle.txt";
//!
//! assert!(glob_match(glob, path));
//! ```
//!
//! ## Validation
//!
//! [`glob_match`] does not report invalid patterns — an unclosed `{` or `[`,
//! a trailing `\`, or brace expansions nested deeper than 10 levels have an
//! unspecified result (typically no match). This is a deliberate performance
//! trade-off: there is no compile step, and the pattern is interpreted lazily
//! while matching, so reliably detecting a malformed pattern would require an
//! extra scan on every call. Validation is instead a separate, one-time step —
//! use [`validate`] to reject such patterns with a descriptive [`Error`]:
//!
//! ```rust
//! use fast_glob::{validate, Error, ErrorKind};
//!
//! assert!(validate("some/**/n*d[k-m]e?txt").is_ok());
//! assert_eq!(
//!     validate("src/**/*.{js,ts"),
//!     Err(Error { kind: ErrorKind::UnclosedBrace, index: 9 })
//! );
//! ```
//!
//! ## Syntax
//!
//! `fast-glob` supports the following glob pattern syntax:
//!
//! | Syntax  | Meaning                                                                                                                                                                                             |
//! | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `?`     | Matches any single character.                                                                                                                                                                       |
//! | `*`     | Matches zero or more characters, except for path separators (e.g., `/`).                                                                                                                             |
//! | `**`    | Matches zero or more characters, including path separators. Must match a complete path segment (i.e., followed by a `/` or the end of the pattern).                                                  |
//! | `[ab]`  | Matches one of the characters contained in the brackets. Character ranges, e.g., `[a-z]`, are also supported. Use `[!ab]` or `[^ab]` to match any character _except_ those contained in the brackets. |
//! | `{a,b}` | Matches one of the patterns contained in the braces. Any of the wildcard characters can be used in the sub-patterns. Braces may be nested up to 10 levels deep.                                     |
//! | `!`     | When at the start of the glob, this negates the result. Multiple `!` characters negate the glob multiple times.                                                                                     |
//! | `\`     | A backslash character may be used to escape any of the above special characters.                                                                                                                    |
//!
//! ---
//!
//! For detailed usage and API reference, refer to the specific function and struct documentation.
//!
//! For any issues or contributions, please visit the [GitHub repository](https://github.com/oxc-project/fast-glob).

/**
 * The following code was originally forked from
 * https://github.com/devongovett/glob-match/blob/d5a6c67/src/lib.rs
 *
 * MIT Licensed
 * Copyright (c) 2023 Devon Govett
 * https://github.com/devongovett/glob-match/tree/main/LICENSE
 */
use std::fmt;
use std::path::is_separator;

use arrayvec::ArrayVec;

const MAX_BRACE_NESTING: usize = 10;

#[derive(Clone, Debug, Default)]
struct State {
    path_index: usize,
    glob_index: usize,
    brace_depth: usize,

    wildcard: Wildcard,
    globstar: Wildcard,
}

#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
    glob_index: u32,
    path_index: u32,
    brace_depth: u32,
}

type BraceStack = ArrayVec<(u32, u32), MAX_BRACE_NESTING>;

/// An error describing why a glob pattern is invalid, returned by [`validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    /// The kind of invalid construct that was found.
    pub kind: ErrorKind,
    /// Byte offset in the pattern of the offending character.
    pub index: usize,
}

/// The kind of invalid construct described by an [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A `{` is never closed by a matching `}`.
    UnclosedBrace,
    /// A `[` is never closed by a matching `]`.
    UnclosedBracket,
    /// A `\` at the end of the pattern has no character to escape.
    TrailingBackslash,
    /// Brace expansions nest deeper than the supported 10 levels.
    BraceNestingTooDeep,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = self.index;
        match self.kind {
            ErrorKind::UnclosedBrace => write!(
                f,
                "unclosed brace expansion at byte {index}; missing '}}' (to match a literal '{{', escape it as '\\{{' or '[{{]')"
            ),
            ErrorKind::UnclosedBracket => write!(
                f,
                "unclosed character class at byte {index}; missing ']' (to match a literal '[', escape it as '\\[' or '[[]')"
            ),
            ErrorKind::TrailingBackslash => write!(
                f,
                "trailing backslash at byte {index} has no character to escape (to match a literal '\\', use '\\\\')"
            ),
            ErrorKind::BraceNestingTooDeep => write!(
                f,
                "brace expansion at byte {index} nests deeper than the supported {MAX_BRACE_NESTING} levels"
            ),
        }
    }
}

impl std::error::Error for Error {}

/// Performs glob pattern matching for `glob` against `path`.
///
/// `glob` is expected to be a valid pattern. An invalid pattern — an unclosed
/// `{` or `[`, a trailing `\`, or brace expansions nested deeper than 10
/// levels — cannot be reported here and its result is unspecified: typically
/// it matches nothing, and it never matches through `!` negation, but the
/// exact behavior may change between releases. Callers accepting user-written
/// patterns should reject invalid ones up front with [`validate`].
pub fn glob_match(glob: impl AsRef<[u8]>, path: impl AsRef<[u8]>) -> bool {
    let (matched, invalid_pattern) = glob_match_internal(glob.as_ref(), path.as_ref());
    matched && !invalid_pattern
}

/// Checks that `glob` is a valid pattern.
///
/// [`glob_match`] has no way to report an invalid pattern, and its result for
/// one is unspecified. Call this once when a pattern is first accepted (e.g.
/// at configuration load time) to reject invalid patterns with an actionable
/// error instead of silently matching nothing.
///
/// A pattern accepted here is never treated as invalid by [`glob_match`], so
/// its matching behavior is well-defined. For a rejected pattern the result
/// of [`glob_match`] is unspecified — typically it matches nothing.
///
/// # Examples
///
/// ```rust
/// use fast_glob::{validate, Error, ErrorKind};
///
/// assert!(validate("some/**/n*d[k-m]e?txt").is_ok());
/// assert_eq!(
///     validate("src/**/*.{js,ts"),
///     Err(Error { kind: ErrorKind::UnclosedBrace, index: 9 })
/// );
/// ```
pub fn validate(glob: impl AsRef<[u8]>) -> Result<(), Error> {
    let glob = glob.as_ref();
    let mut index = 0;

    // Leading `!` characters negate the glob and are not part of the pattern.
    while index < glob.len() && glob[index] == b'!' {
        index += 1;
    }

    let mut open_braces = ArrayVec::<usize, MAX_BRACE_NESTING>::new();

    while index < glob.len() {
        match glob[index] {
            b'\\' => {
                if index + 1 >= glob.len() {
                    return Err(Error { kind: ErrorKind::TrailingBackslash, index });
                }
                index += 2;
            }
            b'[' => match skip_class(glob, index) {
                Some(next) => index = next,
                None => return Err(Error { kind: ErrorKind::UnclosedBracket, index }),
            },
            b'{' => {
                if open_braces.try_push(index).is_err() {
                    return Err(Error { kind: ErrorKind::BraceNestingTooDeep, index });
                }
                index += 1;
            }
            // A `}` without a matching `{` is an ordinary character.
            b'}' => {
                open_braces.pop();
                index += 1;
            }
            _ => index += 1,
        }
    }

    if let Some(&index) = open_braces.first() {
        return Err(Error { kind: ErrorKind::UnclosedBrace, index });
    }

    Ok(())
}

/// Returns the match result (with negation applied) alongside whether the
/// pattern was detected as invalid, so tests can check the latter against
/// [`validate`].
fn glob_match_internal(glob: &[u8], path: &[u8]) -> (bool, bool) {
    let mut state = State::default();

    let mut negated = false;
    while state.glob_index < glob.len() && glob[state.glob_index] == b'!' {
        negated = !negated;
        state.glob_index += 1;
    }

    let mut brace_stack = BraceStack::new();
    let mut invalid_pattern = false;
    let matched = state.glob_match_from(glob, path, 0, &mut brace_stack, &mut invalid_pattern);

    // A negated glob matches every path its pattern does not — for an invalid
    // pattern that would be every path, even when the matcher never reaches
    // the invalid construct (e.g. after an early literal mismatch). Gate the
    // negation flip on validity instead of relying on lazy detection.
    if negated && !matched && !invalid_pattern && validate(glob).is_err() {
        return (false, true);
    }

    (negated ^ matched, invalid_pattern)
}

/// Returns the index just past the `]` closing the character class opened by
/// the `[` at `index`, or `None` if the class is unclosed. Mirrors the class
/// parsing in `glob_match_from`: an optional `^`/`!` prefix, then the first
/// character is a literal member (so a leading `]` does not close the class),
/// and `\` escapes the next character.
fn skip_class(glob: &[u8], index: usize) -> Option<usize> {
    let mut index = index + 1;
    if matches!(glob.get(index), Some(b'^' | b'!')) {
        index += 1;
    }

    let mut first = true;
    loop {
        match glob.get(index)? {
            b']' if !first => return Some(index + 1),
            b'\\' => index += 1,
            _ => {}
        }
        first = false;
        index += 1;
    }
}

#[inline(always)]
fn unescape(c: &mut u8, glob: &[u8], state: &mut State, invalid_pattern: &mut bool) -> bool {
    if *c == b'\\' {
        state.glob_index += 1;
        if state.glob_index >= glob.len() {
            // A trailing backslash has nothing to escape.
            *invalid_pattern = true;
            return false;
        }
        *c = match glob[state.glob_index] {
            b'a' => b'\x61',
            b'b' => b'\x08',
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            c => c,
        }
    }
    true
}

impl State {
    #[inline(always)]
    fn backtrack(&mut self) {
        self.glob_index = self.wildcard.glob_index as usize;
        self.path_index = self.wildcard.path_index as usize;
        self.brace_depth = self.wildcard.brace_depth as usize;
    }

    #[inline(always)]
    fn skip_globstars(&mut self, glob: &[u8]) {
        let mut glob_index = self.glob_index + 2;

        while glob_index + 4 <= glob.len() && &glob[glob_index..glob_index + 4] == b"/**/" {
            glob_index += 3;
        }

        if &glob[glob_index..] == b"/**" {
            glob_index += 3;
        }

        self.glob_index = glob_index - 2;
    }

    #[inline(always)]
    fn skip_to_separator(&mut self, path: &[u8], is_end_invalid: bool) {
        if self.path_index == path.len() {
            self.wildcard.path_index += 1;
            return;
        }

        let mut path_index = self.path_index;
        while path_index < path.len() && !is_separator(path[path_index] as char) {
            path_index += 1;
        }

        if is_end_invalid || path_index != path.len() {
            path_index += 1;
        }

        self.wildcard.path_index = path_index as u32;
        self.globstar = self.wildcard;
    }

    #[inline(always)]
    fn skip_branch(&mut self, glob: &[u8]) {
        let end_brace_depth = self.brace_depth - 1;
        while self.glob_index < glob.len() {
            match glob[self.glob_index] {
                b'{' => self.brace_depth += 1,
                b'}' => {
                    self.brace_depth -= 1;
                    if self.brace_depth == end_brace_depth {
                        self.glob_index += 1;
                        return;
                    }
                }
                b'[' => {
                    // An unclosed class swallows the rest of the glob.
                    self.glob_index = skip_class(glob, self.glob_index).unwrap_or(glob.len());
                    continue;
                }
                b'\\' => self.glob_index += 1,
                _ => (),
            }
            self.glob_index += 1;
        }
    }

    fn match_brace_branch(
        &self,
        glob: &[u8],
        path: &[u8],
        open_brace_index: usize,
        branch_index: usize,
        brace_stack: &mut BraceStack,
        invalid_pattern: &mut bool,
    ) -> bool {
        // Gracefully reject brace expansions deeper than BraceStack capacity.
        if brace_stack.try_push((open_brace_index as u32, branch_index as u32)).is_err() {
            *invalid_pattern = true;
            return false;
        }

        let mut branch_state = self.clone();
        branch_state.glob_index = branch_index;
        branch_state.brace_depth = brace_stack.len();

        let matched =
            branch_state.glob_match_from(glob, path, branch_index, brace_stack, invalid_pattern);

        brace_stack.pop();

        matched
    }

    fn match_brace(
        &mut self,
        glob: &[u8],
        path: &[u8],
        brace_stack: &mut BraceStack,
        invalid_pattern: &mut bool,
    ) -> bool {
        let mut brace_depth = 0;
        let mut has_closing_brace = false;
        let mut matched = false;

        let open_brace_index = self.glob_index;

        let mut branch_index = 0;

        while self.glob_index < glob.len() {
            match glob[self.glob_index] {
                b'{' => {
                    brace_depth += 1;
                    if brace_depth == 1 {
                        branch_index = self.glob_index + 1;
                    }
                }
                b'}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        has_closing_brace = true;
                        if self.match_brace_branch(
                            glob,
                            path,
                            open_brace_index,
                            branch_index,
                            brace_stack,
                            invalid_pattern,
                        ) {
                            matched = true;
                        }
                        break;
                    }
                }
                b',' if brace_depth == 1 => {
                    if self.match_brace_branch(
                        glob,
                        path,
                        open_brace_index,
                        branch_index,
                        brace_stack,
                        invalid_pattern,
                    ) {
                        matched = true;
                    }
                    branch_index = self.glob_index + 1;
                }
                b'[' => {
                    // An unclosed class swallows the rest of the glob,
                    // leaving the brace unclosed as well.
                    self.glob_index = skip_class(glob, self.glob_index).unwrap_or(glob.len());
                    continue;
                }
                b'\\' => self.glob_index += 1,
                _ => (),
            }
            self.glob_index += 1;
        }

        if !has_closing_brace {
            *invalid_pattern = true;
            return false;
        }

        matched
    }

    #[inline(always)]
    fn glob_match_from(
        &mut self,
        glob: &[u8],
        path: &[u8],
        match_start: usize,
        brace_stack: &mut BraceStack,
        invalid_pattern: &mut bool,
    ) -> bool {
        while self.glob_index < glob.len() || self.path_index < path.len() {
            if self.glob_index < glob.len() {
                match glob[self.glob_index] {
                    b'*' => {
                        let is_globstar =
                            self.glob_index + 1 < glob.len() && glob[self.glob_index + 1] == b'*';
                        if is_globstar {
                            self.skip_globstars(glob);
                        }

                        self.wildcard.glob_index = self.glob_index as u32;
                        self.wildcard.path_index = self.path_index as u32 + 1;
                        self.wildcard.brace_depth = self.brace_depth as u32;

                        let mut in_globstar = false;
                        if is_globstar {
                            self.glob_index += 2;

                            let is_end_invalid = self.glob_index != glob.len();

                            if (self.glob_index.saturating_sub(match_start) < 3
                                || glob[self.glob_index - 3] == b'/')
                                && (!is_end_invalid || glob[self.glob_index] == b'/')
                            {
                                if is_end_invalid {
                                    self.glob_index += 1;
                                }

                                self.skip_to_separator(path, is_end_invalid);
                                in_globstar = true;
                            }
                        } else {
                            self.glob_index += 1;
                        }

                        if !in_globstar
                            && self.path_index < path.len()
                            && is_separator(path[self.path_index] as char)
                        {
                            self.wildcard = self.globstar;
                        }

                        continue;
                    }
                    b'?' if self.path_index < path.len()
                        && !is_separator(path[self.path_index] as char) =>
                    {
                        self.glob_index += 1;
                        self.path_index += 1;
                        continue;
                    }
                    b'[' if self.path_index < path.len() => {
                        self.glob_index += 1;

                        let mut negated = false;
                        if self.glob_index < glob.len()
                            && matches!(glob[self.glob_index], b'^' | b'!')
                        {
                            negated = true;
                            self.glob_index += 1;
                        }

                        let mut first = true;
                        let mut is_match = false;
                        let c = path[self.path_index];
                        while self.glob_index < glob.len()
                            && (first || glob[self.glob_index] != b']')
                        {
                            let mut low = glob[self.glob_index];
                            if !unescape(&mut low, glob, self, invalid_pattern) {
                                return false;
                            }

                            self.glob_index += 1;

                            let high = if self.glob_index + 1 < glob.len()
                                && glob[self.glob_index] == b'-'
                                && glob[self.glob_index + 1] != b']'
                            {
                                self.glob_index += 1;

                                let mut high = glob[self.glob_index];
                                if !unescape(&mut high, glob, self, invalid_pattern) {
                                    return false;
                                }

                                self.glob_index += 1;
                                high
                            } else {
                                low
                            };

                            if low <= c && c <= high {
                                is_match = true;
                            }

                            first = false;
                        }

                        if self.glob_index >= glob.len() {
                            *invalid_pattern = true;
                            return false;
                        }

                        self.glob_index += 1;
                        if is_match != negated {
                            self.path_index += 1;
                            continue;
                        }
                    }
                    b'{' => {
                        if let Some((_, branch_index)) =
                            brace_stack.iter().find(|(open_brace_index, _)| {
                                *open_brace_index == self.glob_index as u32
                            })
                        {
                            self.glob_index = *branch_index as usize;
                            self.brace_depth += 1;
                            continue;
                        }
                        return self.match_brace(glob, path, brace_stack, invalid_pattern);
                    }
                    b',' | b'}' if self.brace_depth > 0 => {
                        self.skip_branch(glob);
                        continue;
                    }
                    mut c if self.path_index < path.len() => {
                        if !unescape(&mut c, glob, self, invalid_pattern) {
                            return false;
                        }

                        let is_match = if c == b'/' {
                            is_separator(path[self.path_index] as char)
                        } else {
                            path[self.path_index] == c
                        };

                        if is_match {
                            self.glob_index += 1;
                            self.path_index += 1;

                            if c == b'/' {
                                self.wildcard = self.globstar;
                            }

                            continue;
                        }
                    }
                    _ => {}
                }
            }

            if self.wildcard.path_index > 0 && self.wildcard.path_index <= path.len() as u32 {
                self.backtrack();
                continue;
            }

            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALPHABET: &[u8] = b"a/*[]{}\\,!-";

    fn for_each_pattern(len: usize, f: &mut impl FnMut(&[u8])) {
        let mut pattern = vec![0u8; len];
        for mut n in 0..ALPHABET.len().pow(len as u32) {
            for slot in &mut pattern {
                *slot = ALPHABET[n % ALPHABET.len()];
                n /= ALPHABET.len();
            }
            f(&pattern);
        }
    }

    /// `validate` and the matcher must agree on what is invalid:
    /// a pattern `validate` accepts is never flagged invalid by the matcher,
    /// and an invalid non-negated pattern never matches any path.
    /// Checked exhaustively over all short patterns built from the special characters.
    #[test]
    fn validate_agrees_with_matcher() {
        const PATHS: &[&str] = &["", "a", "aa", "a/a", "-", ",", "!"];

        for len in 0..=6 {
            for_each_pattern(len, &mut |pattern| {
                let valid = validate(pattern).is_ok();
                for path in PATHS {
                    let (matched, invalid) = glob_match_internal(pattern, path.as_bytes());
                    if valid {
                        assert!(
                            !invalid,
                            "matcher flagged {:?} as invalid on path {path:?} but validate accepted it",
                            String::from_utf8_lossy(pattern),
                        );
                    } else if pattern.first() == Some(&b'!') || !pattern.contains(&b'{') {
                        // A negated invalid pattern never matches (the negation flip is gated on validity),
                        // and neither does a brace-free one, since every part of its glob is processed directly.
                        // (A non-negated invalid construct in a non-taken brace branch may go unnoticed,
                        // that behavior is documented as unspecified.)
                        assert!(
                            !(matched && !invalid),
                            "invalid pattern {:?} matched path {path:?}",
                            String::from_utf8_lossy(pattern),
                        );
                    }
                }
            });
        }
    }
}
