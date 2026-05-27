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
//! For any issues or contributions, please visit the [GitHub repository](https://github.com/oxc-project/fast-glob).

/**
 * The following code is modified based on
 * https://github.com/devongovett/glob-match/blob/d5a6c67/src/lib.rs
 *
 * MIT Licensed
 * Copyright (c) 2023 Devon Govett
 * https://github.com/devongovett/glob-match/tree/main/LICENSE
 */
use arrayvec::ArrayVec;

/// Whether `c` is safe to use as a literal anchor character when fast-
/// forwarding the path after `**`. Any of the glob metacharacters could
/// expand to something other than this single byte, so they aren't safe.
#[inline(always)]
fn is_literal_anchor(c: u8) -> bool {
    !matches!(c, b'*' | b'?' | b'[' | b'{' | b'\\' | b',' | b'}' | b'/')
}

#[inline(always)]
fn is_separator(c: u8) -> bool {
    #[cfg(windows)]
    {
        c == b'/' || c == b'\\'
    }
    #[cfg(not(windows))]
    {
        c == b'/'
    }
}

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

type BraceStack = ArrayVec<(u32, u32), 10>;

pub fn glob_match(glob: impl AsRef<[u8]>, path: impl AsRef<[u8]>) -> bool {
    let glob = glob.as_ref();
    let path = path.as_ref();
    glob_match_impl(glob, path)
}

fn glob_match_impl(glob: &[u8], path: &[u8]) -> bool {
    let mut state = State::default();

    let mut negated = false;
    while state.glob_index < glob.len() && glob[state.glob_index] == b'!' {
        negated = !negated;
        state.glob_index += 1;
    }

    let mut brace_stack = ArrayVec::<_, 10>::new();
    let mut invalid_pattern = false;
    let matched = state.glob_match_from(glob, path, 0, &mut brace_stack, &mut invalid_pattern);
    if invalid_pattern {
        return false;
    }

    negated ^ matched
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
        while path_index < path.len() && !is_separator(path[path_index]) {
            path_index += 1;
        }

        if is_end_invalid || path_index != path.len() {
            path_index += 1;
        }

        self.wildcard.path_index = path_index as u32;
        self.globstar = self.wildcard;
    }

    /// Skip ahead past zero or more path segments to find the next position
    /// where the literal `anchor` appears right after a separator (or at the
    /// current position). This collapses what would otherwise be N rounds of
    /// `**` backtracking — for `**/n*d`, instead of trying `n` at every
    /// `/`-anchored position by re-entering the wildcard arm, jump directly
    /// to the next plausible position in one pass.
    ///
    /// `wildcard.path_index` is then set to the next backtrack candidate
    /// (past the next separator after the match, or past end), so that on
    /// failure the next round of `**` matching resumes from a different
    /// segment — matching the semantics of repeated `skip_to_separator`.
    #[inline(always)]
    fn skip_to_anchor(&mut self, path: &[u8], anchor: u8, is_end_invalid: bool) {
        let path_len = path.len();
        let mut p = self.path_index;
        let mut found_pos: Option<usize> = None;

        // Check current position first — `**` may match zero chars.
        if p < path_len && unsafe { *path.get_unchecked(p) } == anchor {
            found_pos = Some(p);
            p += 1;
        } else {
            loop {
                while p < path_len && !is_separator(unsafe { *path.get_unchecked(p) }) {
                    p += 1;
                }
                if p >= path_len {
                    break;
                }
                p += 1;
                if p < path_len && unsafe { *path.get_unchecked(p) } == anchor {
                    found_pos = Some(p);
                    p += 1;
                    break;
                }
            }
        }

        // From wherever we ended up (past the matched anchor, or end), find
        // the next separator and set wildcard.path_index = past it. This is
        // the same convention used by `skip_to_separator`.
        while p < path_len && !is_separator(unsafe { *path.get_unchecked(p) }) {
            p += 1;
        }
        if p < path_len || is_end_invalid {
            p += 1;
        }

        if let Some(matched) = found_pos {
            self.path_index = matched;
        }
        self.wildcard.path_index = p as u32;
        self.globstar = self.wildcard;
    }

    #[inline(always)]
    fn skip_branch(&mut self, glob: &[u8]) {
        let mut in_brackets = false;
        let end_brace_depth = self.brace_depth - 1;
        while self.glob_index < glob.len() {
            match glob[self.glob_index] {
                b'{' if !in_brackets => self.brace_depth += 1,
                b'}' if !in_brackets => {
                    self.brace_depth -= 1;
                    if self.brace_depth == end_brace_depth {
                        self.glob_index += 1;
                        return;
                    }
                }
                b'[' if !in_brackets => in_brackets = true,
                b']' => in_brackets = false,
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
        let mut in_brackets = false;
        let mut has_closing_brace = false;
        let mut matched = false;

        let open_brace_index = self.glob_index;

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
                b'[' if !in_brackets => in_brackets = true,
                b']' => in_brackets = false,
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

    fn glob_match_from(
        &mut self,
        glob: &[u8],
        path: &[u8],
        match_start: usize,
        brace_stack: &mut BraceStack,
        invalid_pattern: &mut bool,
    ) -> bool {
        let glob_len = glob.len();
        let path_len = path.len();

        while self.glob_index < glob_len || self.path_index < path_len {
            if self.glob_index < glob_len {
                // SAFETY: We just checked `self.glob_index < glob_len`.
                let gc = unsafe { *glob.get_unchecked(self.glob_index) };
                match gc {
                    b'*' => {
                        let is_globstar = self.glob_index + 1 < glob_len
                            && unsafe { *glob.get_unchecked(self.glob_index + 1) } == b'*';
                        if is_globstar {
                            self.skip_globstars(glob);
                        }

                        self.wildcard.glob_index = self.glob_index as u32;
                        self.wildcard.path_index = self.path_index as u32 + 1;
                        self.wildcard.brace_depth = self.brace_depth as u32;

                        let mut in_globstar = false;
                        if is_globstar {
                            self.glob_index += 2;

                            let is_end_invalid = self.glob_index != glob_len;

                            let prefix_ok = self.glob_index < match_start + 3
                                || unsafe { *glob.get_unchecked(self.glob_index - 3) } == b'/';
                            let suffix_ok = !is_end_invalid
                                || unsafe { *glob.get_unchecked(self.glob_index) } == b'/';

                            if prefix_ok && suffix_ok {
                                if is_end_invalid {
                                    self.glob_index += 1;
                                }

                                // If the char immediately after `**/` is a plain
                                // literal, search for the next `/<literal>` in the
                                // path directly. This collapses the otherwise
                                // O(segments) backtracking loop into one pass.
                                let anchor_byte = if self.glob_index < glob_len {
                                    let next = unsafe { *glob.get_unchecked(self.glob_index) };
                                    if is_literal_anchor(next) {
                                        Some(next)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                };

                                if let Some(anchor) = anchor_byte {
                                    self.skip_to_anchor(path, anchor, is_end_invalid);
                                } else {
                                    self.skip_to_separator(path, is_end_invalid);
                                }
                                in_globstar = true;
                            }
                        } else {
                            self.glob_index += 1;
                        }

                        if !in_globstar
                            && self.path_index < path_len
                            && is_separator(unsafe { *path.get_unchecked(self.path_index) })
                        {
                            self.wildcard = self.globstar;
                        }

                        continue;
                    }
                    b'?' if self.path_index < path_len
                        && !is_separator(unsafe { *path.get_unchecked(self.path_index) }) =>
                    {
                        self.glob_index += 1;
                        self.path_index += 1;
                        continue;
                    }
                    b'[' if self.path_index < path_len => {
                        self.glob_index += 1;

                        let mut negated = false;
                        if self.glob_index < glob_len
                            && matches!(
                                unsafe { *glob.get_unchecked(self.glob_index) },
                                b'^' | b'!'
                            )
                        {
                            negated = true;
                            self.glob_index += 1;
                        }

                        let mut first = true;
                        let mut is_match = false;
                        let c = unsafe { *path.get_unchecked(self.path_index) };
                        while self.glob_index < glob_len
                            && (first || unsafe { *glob.get_unchecked(self.glob_index) } != b']')
                        {
                            let mut low = unsafe { *glob.get_unchecked(self.glob_index) };
                            if !unescape(&mut low, glob, self) {
                                return false;
                            }

                            self.glob_index += 1;

                            let high = if self.glob_index + 1 < glob_len
                                && unsafe { *glob.get_unchecked(self.glob_index) } == b'-'
                                && unsafe { *glob.get_unchecked(self.glob_index + 1) } != b']'
                            {
                                self.glob_index += 1;

                                let mut high = unsafe { *glob.get_unchecked(self.glob_index) };
                                if !unescape(&mut high, glob, self) {
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

                        if self.glob_index >= glob_len {
                            return false;
                        }

                        self.glob_index += 1;
                        if is_match != negated {
                            self.path_index += 1;
                            continue;
                        }
                    }
                    b'{' => {
                        let target = self.glob_index as u32;
                        if let Some((_, branch_index)) = brace_stack
                            .iter()
                            .find(|(open_brace_index, _)| *open_brace_index == target)
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
                    mut c if self.path_index < path_len => {
                        if !unescape(&mut c, glob, self) {
                            return false;
                        }

                        let pc = unsafe { *path.get_unchecked(self.path_index) };
                        let is_match = if c == b'/' { is_separator(pc) } else { pc == c };

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

            if self.wildcard.path_index > 0 && self.wildcard.path_index <= path_len as u32 {
                self.backtrack();
                continue;
            }

            return false;
        }

        true
    }
}
