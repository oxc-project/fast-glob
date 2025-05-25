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
mine                       time:   [84.413 ns 84.548 ns 84.661 ns]
glob                       time:   [398.63 ns 399.36 ns 400.10 ns]
globset                    time:   [30.919 µs 30.942 µs 30.976 µs]
glob_match                 time:   [224.16 ns 224.57 ns 225.03 ns]
glob_pre_compiled          time:   [78.929 ns 79.362 ns 79.801 ns]
globset_pre_compiled       time:   [103.17 ns 103.22 ns 103.27 ns]
wax                        time:   [84.712 µs 84.831 µs 84.953 µs]
wax-pre-compiled           time:   [43.661 ns 43.679 ns 43.701 ns]
```

### Test Case 2

```rust
const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";
```

```
mine                       time:   [188.01 ns 188.40 ns 188.79 ns]
globset                    time:   [38.565 µs 38.684 µs 38.841 µs]
glob_match                 time:   [381.81 ns 383.12 ns 384.43 ns]
globset_pre_compiled       time:   [103.29 ns 103.35 ns 103.42 ns]
wax                        time:   [107.04 µs 107.38 µs 107.78 µs]
wax-pre-compiled           time:   [43.665 ns 43.764 ns 43.918 ns]
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
