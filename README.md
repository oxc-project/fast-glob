# fast-glob

## Introduce

A high-performance glob matching crate for Rust based on [`devongovett/glob-match`](https://github.com/devongovett/glob-match).

**Key Features:**

- Up to 60% performance improvement.
- Supports more complete and well-rounded features.

## Examples

```rust
use fast_glob::glob_match;

let glob = "some/**/n*d[k-m]e?txt";
let path = "some/a/bigger/path/to/the/crazy/needle.txt";

assert!(glob_match(glob, path));
```

## Syntax

| Syntax  | Meaning                                                                                                                                                                                             |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`     | Matches any single character.                                                                                                                                                                       |
| `*`     | Matches zero or more characters, except for path separators (e.g. `/`).                                                                                                                             |
| `**`    | Matches zero or more characters, including path separators. Must match a complete path segment (i.e. followed by a `/` or the end of the pattern).                                                  |
| `[ab]`  | Matches one of the characters contained in the brackets. Character ranges, e.g. `[a-z]` are also supported. Use `[!ab]` or `[^ab]` to match any character _except_ those contained in the brackets. |
| `{a,b}` | Matches one of the patterns contained in the braces. Any of the wildcard characters can be used in the sub-patterns. Braces may be nested up to 10 levels deep.                                     |
| `!`     | When at the start of the glob, this negates the result. Multiple `!` characters negate the glob multiple times.                                                                                     |
| `\`     | A backslash character may be used to escape any of the above special characters.                                                                                                                    |

## Benchmark

### Test Case 1

```rust
const GLOB: &'static str = "some/**/n*d[k-m]e?txt";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [87.185 ns 87.307 ns 87.440 ns]
glob                       time:   [376.83 ns 377.42 ns 378.09 ns]
globset                    time:   [21.027 µs 21.035 µs 21.045 µs]
glob_match                 time:   [203.66 ns 203.87 ns 204.09 ns]
glob_pre_compiled          time:   [63.569 ns 63.684 ns 63.800 ns]
globset_pre_compiled       time:   [91.543 ns 91.591 ns 91.651 ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [198.63 ns 199.26 ns 200.08 ns]
globset                    time:   [41.489 µs 41.575 µs 41.681 µs]
glob_match                 time:   [367.32 ns 368.10 ns 368.77 ns]
globset_pre_compiled       time:   [91.498 ns 91.648 ns 91.883 ns]
```

## FAQ

### Why not use the more efficient `glob_match` for brace expansion?

`glob_match` is unable to handle complex brace expansions. Below are some failed examples:

- `glob_match("{a/b,a/b/c}/c", "a/b/c")`
- `glob_match("**/foo{bar,b*z}", "foobuzz")`
- `glob_match("**/{a,b}/c.png", "some/a/b/c.png")`

Due to these limitations, `brace expansion` requires a different implementation that can handle the complexity of such patterns, without any performance compromise.

## Credits

- The [glob-match](https://github.com/devongovett/glob-match) project created by [@devongovett](https://github.com/devongovett) which is an extremely fast glob matching library in Rust.
