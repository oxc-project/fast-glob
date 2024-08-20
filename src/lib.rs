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
//! For any issues or contributions, please visit the [GitHub repository](https://github.com/shulaoda/fast-glob).
//!

/// Performs glob pattern matching for a simple glob pattern.
///
/// Returns `true` if `glob` matches `path`, `false` otherwise.
///
/// # Example
///
/// ```
/// use fast_glob::glob_match;
///
/// let glob = "**/*.txt";
/// let path = "file.txt";
///
/// assert!(glob_match(glob, path));
/// ```
pub fn glob_match(glob: &str, path: &str) -> bool {
  let glob = glob.as_bytes();
  let path = path.as_bytes();

  let mut state = State::default();

  let mut negated = false;
  while state.glob_index < glob.len() && glob[state.glob_index] == b'!' {
    negated = !negated;
    state.glob_index += 1;
  }

  let matched = glob_match_from(glob, path, &mut state);
  if negated {
    !matched
  } else {
    matched
  }
}

/**
 * The following code is modified based on
 * https://github.com/devongovett/glob-match/blob/d5a6c67/src/lib.rs
 *
 * MIT Licensed
 * Copyright (c) 2023 Devon Govett
 * https://github.com/devongovett/glob-match/tree/main/LICENSE
 */
use std::path::is_separator;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct State {
  pub(crate) path_index: usize,
  pub(crate) glob_index: usize,

  wildcard: Wildcard,
  globstar: Wildcard,
}

#[derive(Clone, Copy, Debug, Default)]
struct Wildcard {
  glob_index: usize,
  path_index: usize,
}

#[inline(always)]
fn unescape(c: &mut u8, glob: &[u8], state: &mut State) -> bool {
  if *c == b'\\' {
    state.glob_index += 1;
    if state.glob_index >= glob.len() {
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
    self.glob_index = self.wildcard.glob_index;
    self.path_index = self.wildcard.path_index;
  }

  #[inline(always)]
  fn skip_globstars(&mut self, glob: &[u8]) {
    let mut glob_index = self.glob_index + 2;

    while glob_index + 4 <= glob.len() && &glob[glob_index..glob_index + 4] == b"/**/" {
      glob_index += 3;
    }

    if glob_index + 3 == glob.len() && &glob[glob_index..] == b"/**" {
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

    self.wildcard.path_index = path_index;
    self.globstar = self.wildcard;
  }

  fn match_brace_branch(
    &self,
    glob: &[u8],
    path: &[u8],
    open_brace_index: usize,
    close_brace_index: usize,
    branch_index: usize,
    buffer: &mut Vec<u8>,
  ) -> bool {
    buffer.extend_from_slice(&glob[..open_brace_index]);
    buffer.extend_from_slice(&glob[branch_index..self.glob_index]);
    buffer.extend_from_slice(&glob[close_brace_index + 1..]);

    let mut branch_state = self.clone();
    branch_state.glob_index = open_brace_index;
    let matched = glob_match_from(&buffer, path, &mut branch_state);
    buffer.clear();
    matched
  }

  fn match_brace(&mut self, glob: &[u8], path: &[u8]) -> bool {
    let mut brace_depth = 0;
    let mut in_brackets = false;

    let open_brace_index = self.glob_index;
    let mut close_brace_index = 0;
    let mut glob_index = self.glob_index;
    while glob_index < glob.len() {
      match glob[glob_index] {
        b'{' if !in_brackets => brace_depth += 1,
        b'}' if !in_brackets => {
          brace_depth -= 1;
          if brace_depth == 0 {
            close_brace_index = glob_index;
            break;
          }
        }
        b'[' if !in_brackets => in_brackets = true,
        b']' => in_brackets = false,
        b'\\' => glob_index += 1,
        _ => (),
      }
      glob_index += 1;
    }
    if brace_depth != 0 {
      // Invalid pattern!
      return false;
    }

    let mut buffer = Vec::with_capacity(glob.len());

    let mut branch_index = 0;
    while self.glob_index < glob.len() {
      match glob[self.glob_index] {
        b'{' if !in_brackets => {
          brace_depth += 1;
          if brace_depth == 1 {
            branch_index = self.glob_index + 1;
          }
        }
        b'}' if !in_brackets => {
          brace_depth -= 1;
          if brace_depth == 0 {
            if self.match_brace_branch(
              glob,
              path,
              open_brace_index,
              close_brace_index,
              branch_index,
              &mut buffer,
            ) {
              return true;
            }
            break;
          }
        }
        b',' if brace_depth == 1 => {
          if self.match_brace_branch(
            glob,
            path,
            open_brace_index,
            close_brace_index,
            branch_index,
            &mut buffer,
          ) {
            return true;
          }
          branch_index = self.glob_index + 1;
        }
        b'[' if !in_brackets => in_brackets = true,
        b']' => in_brackets = false,
        b'\\' => self.glob_index += 1,
        _ => (),
      }
      self.glob_index += 1;
    }
    return false;
  }
}

fn glob_match_from(glob: &[u8], path: &[u8], state: &mut State) -> bool {
  while state.glob_index < glob.len() || state.path_index < path.len() {
    if state.glob_index < glob.len() {
      match glob[state.glob_index] {
        b'*' => {
          let is_globstar = state.glob_index + 1 < glob.len() && glob[state.glob_index + 1] == b'*';
          if is_globstar {
            state.skip_globstars(glob);
          }

          state.wildcard.glob_index = state.glob_index;
          state.wildcard.path_index = state.path_index + 1;

          let mut in_globstar = false;
          if is_globstar {
            state.glob_index += 2;

            let is_end_invalid = state.glob_index != glob.len();

            if (state.glob_index < 3 || glob[state.glob_index - 3] == b'/')
              && (!is_end_invalid || glob[state.glob_index] == b'/')
            {
              if is_end_invalid {
                state.glob_index += 1;
              }

              state.skip_to_separator(path, is_end_invalid);
              in_globstar = true;
            }
          } else {
            state.glob_index += 1;
          }

          if !in_globstar
            && state.path_index < path.len()
            && is_separator(path[state.path_index] as char)
          {
            state.wildcard = state.globstar;
          }

          continue;
        }
        b'?' if state.path_index < path.len() => {
          if !is_separator(path[state.path_index] as char) {
            state.glob_index += 1;
            state.path_index += 1;
            continue;
          }
        }
        b'[' if state.path_index < path.len() => {
          state.glob_index += 1;

          let mut negated = false;
          if state.glob_index < glob.len() && matches!(glob[state.glob_index], b'^' | b'!') {
            negated = true;
            state.glob_index += 1;
          }

          let mut first = true;
          let mut is_match = false;
          let c = path[state.path_index];
          while state.glob_index < glob.len() && (first || glob[state.glob_index] != b']') {
            let mut low = glob[state.glob_index];
            if !unescape(&mut low, glob, state) {
              return false;
            }

            state.glob_index += 1;

            let high = if state.glob_index + 1 < glob.len()
              && glob[state.glob_index] == b'-'
              && glob[state.glob_index + 1] != b']'
            {
              state.glob_index += 1;

              let mut high = glob[state.glob_index];
              if !unescape(&mut high, glob, state) {
                return false;
              }

              state.glob_index += 1;
              high
            } else {
              low
            };

            if low <= c && c <= high {
              is_match = true;
            }

            first = false;
          }

          if state.glob_index >= glob.len() {
            return false;
          }

          state.glob_index += 1;
          if is_match != negated {
            state.path_index += 1;
            continue;
          }
        }
        b'{' if state.path_index < path.len() => {
          return state.match_brace(glob, path);
        }
        mut c if state.path_index < path.len() => {
          if !unescape(&mut c, glob, state) {
            return false;
          }

          let is_match = if c == b'/' {
            is_separator(path[state.path_index] as char)
          } else {
            path[state.path_index] == c
          };

          if is_match {
            state.glob_index += 1;
            state.path_index += 1;

            if c == b'/' {
              state.wildcard = state.globstar;
            }

            continue;
          }
        }
        _ => {}
      }
    }

    if state.wildcard.path_index > 0 && state.wildcard.path_index <= path.len() {
      state.backtrack();
      continue;
    }

    return false;
  }

  return true;
}
