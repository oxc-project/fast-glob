use fast_glob::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_input() {
        assert!(glob_match("**/*", "foo"));
        assert!(glob_match("**/*", "foo"));
        assert!(glob_match("**/*", "foo"));
        assert!(glob_match("**/*", "foo"));
        assert!(glob_match("**/*", "foo"));

        assert!(glob_match("**/*".as_bytes(), "foo"));
        assert!(glob_match("**/*".as_bytes(), "foo".as_bytes()));
        assert!(glob_match("**/*".as_bytes(), "foo"));
    }

    #[test]
    fn webpack() {
        // Match everything
        assert!(glob_match("**/*", "foo"));

        // Match the end
        assert!(glob_match("**/f*", "foo"));

        // Match the start
        assert!(glob_match("**/*o", "foo"));

        // Match the middle
        assert!(glob_match("**/f*uck", "firetruck"));

        // Don't match without Regexp 'g'
        assert!(!glob_match("**/uc", "firetruck"));

        // Match zero characters
        assert!(glob_match("**/f*uck", "fuck"));

        // More complex matches
        assert!(glob_match("**/*.min.js", "some/jquery.min.js"));
        assert!(glob_match("**/*.min.*", "some/jquery.min.js"));
        assert!(glob_match("*/js/*.js", "some/js/jquery.min.js"));

        // More complex matches with RegExp 'g' flag (complex regression)
        assert!(glob_match("**/*.min.*", "some/jquery.min.js"));
        assert!(glob_match("**/*.min.js", "some/jquery.min.js"));
        assert!(glob_match("*/js/*.js", "some/js/jquery.min.js"));

        assert!(!glob_match("\\\\/$^+?.()=!|{},[].*", "\\/$^+?.()=!|{},[].*"));

        // Equivalent matches without/with using RegExp 'g'
        assert!(!glob_match("**/.min.", "some/jquery.min.js"));
        assert!(glob_match("**/*.min.*", "some/jquery.min.js"));
        assert!(!glob_match("**/.min.", "some/jquery.min.js"));

        assert!(!glob_match("**/min.js", "some/jquery.min.js"));
        assert!(glob_match("**/*.min.js", "some/jquery.min.js"));
        assert!(!glob_match("**/min.js", "some/jquery.min.js"));

        // Match anywhere (globally) using RegExp 'g'
        assert!(!glob_match("**/min", "some/jquery.min.js"));
        assert!(!glob_match("/js/", "some/js/jquery.min.js"));

        assert!(!glob_match("/js*jq*.js", "some/js/jquery.min.js"));

        // Extended mode

        // ?: Match one character, no more and no less
        assert!(glob_match("**/f?o", "foo"));
        assert!(!glob_match("**/f?o", "fooo"));
        assert!(!glob_match("**/f?oo", "foo"));

        // ?: Match one character with RegExp 'g'
        assert!(glob_match("**/f?o", "foo"));
        assert!(!glob_match("**/f?o", "fooo"));
        assert!(glob_match("**/f?o?", "fooo"));
        assert!(!glob_match("**/?fo", "fooo"));
        assert!(!glob_match("**/f?oo", "foo"));
        assert!(!glob_match("**/foo?", "foo"));

        // []: Match a character range
        assert!(glob_match("**/fo[oz]", "foo"));
        assert!(glob_match("**/fo[oz]", "foz"));
        assert!(!glob_match("**/fo[oz]", "fog"));

        // []: Match a character range and RegExp 'g' (regresion)
        assert!(glob_match("**/fo[oz]", "foo"));
        assert!(glob_match("**/fo[oz]", "foz"));
        assert!(!glob_match("**/fo[oz]", "fog"));

        // {}: Match a choice of different substrings
        assert!(glob_match("**/foo{bar,baaz}", "foobaaz"));
        assert!(glob_match("**/foo{bar,baaz}", "foobar"));
        assert!(!glob_match("**/foo{bar,baaz}", "foobuzz"));
        assert!(glob_match("**/foo{bar,b*z}", "foobuzz"));

        // {}: Match a choice of different substrings and RegExp 'g' (regression)
        assert!(glob_match("**/foo{bar,baaz}", "foobaaz"));
        assert!(glob_match("**/foo{bar,baaz}", "foobar"));
        assert!(!glob_match("**/foo{bar,baaz}", "foobuzz"));

        // More complex extended matches
        assert!(glob_match("?o[oz].b*z.com/{*.js,*.html}", "foo.baaz.com/jquery.min.js"));
        assert!(glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.buzz.com/index.html"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.buzz.com/index.htm"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.bar.com/index.html"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "flozz.buzz.com/index.html"));

        // More complex extended matches and RegExp 'g' (regresion)
        assert!(glob_match("?o[oz].b*z.com/{*.js,*.html}", "foo.baaz.com/jquery.min.js"));
        assert!(glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.buzz.com/index.html"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.buzz.com/index.htm"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "moz.bar.com/index.html"));
        assert!(!glob_match("?o[oz].b*z.com/{*.js,*.html}", "flozz.buzz.com/index.html"));

        // globstar
        assert!(glob_match("some/**/{*.js,*.html}", "some/bar/jquery.min.js"));
        assert!(glob_match("some/**/{*.js,*.html}", "some/bar/baz/jquery.min.js"));
        assert!(glob_match("some/**", "some/bar/baz/jquery.min.js"));

        assert!(glob_match("\\\\/$^+.()=!|,.*", "\\/$^+.()=!|,.*"));

        // globstar specific tests
        assert!(glob_match("/foo/*", "/foo/bar.txt"));
        assert!(glob_match("/foo/**", "/foo/baz.txt"));
        assert!(glob_match("/foo/**", "/foo/bar/baz.txt"));
        assert!(glob_match("/foo/*/*.txt", "/foo/bar/baz.txt"));
        assert!(glob_match("/foo/**/*.txt", "/foo/bar/baz.txt"));
        assert!(glob_match("/foo/**/*.txt", "/foo/bar/baz/qux.txt"));
        assert!(glob_match("/foo/**/bar.txt", "/foo/bar.txt"));
        assert!(glob_match("/foo/**/**/bar.txt", "/foo/bar.txt"));
        assert!(glob_match("/foo/**/*/baz.txt", "/foo/bar/baz.txt"));
        assert!(glob_match("/foo/**/*.txt", "/foo/bar.txt"));
        assert!(glob_match("/foo/**/**/*.txt", "/foo/bar.txt"));
        assert!(glob_match("/foo/**/*/*.txt", "/foo/bar/baz.txt"));
        assert!(glob_match("**/*.txt", "/foo/bar/baz/qux.txt"));
        assert!(glob_match("**/foo.txt", "foo.txt"));
        assert!(glob_match("**/*.txt", "foo.txt"));

        assert!(!glob_match("/foo/*", "/foo/bar/baz.txt"));
        assert!(!glob_match("/foo/*.txt", "/foo/bar/baz.txt"));
        assert!(!glob_match("/foo/*/*.txt", "/foo/bar/baz/qux.txt"));
        assert!(!glob_match("/foo/*/bar.txt", "/foo/bar.txt"));
        assert!(!glob_match("/foo/*/*/baz.txt", "/foo/bar/baz.txt"));
        assert!(!glob_match("/foo/**.txt", "/foo/bar/baz/qux.txt"));
        assert!(!glob_match("/foo/bar**/*.txt", "/foo/bar/baz/qux.txt"));
        assert!(!glob_match("/foo/bar**", "/foo/bar/baz.txt"));
        assert!(!glob_match("**/.txt", "/foo/bar/baz/qux.txt"));
        assert!(!glob_match("*/*.txt", "/foo/bar/baz/qux.txt"));
        assert!(!glob_match("*/*.txt", "foo.txt"));

        assert!(!glob_match("some/*", "some/bar/baz/jquery.min.js"));

        assert!(!glob_match("some/*", "some/bar/baz/jquery.min.js"));
        assert!(glob_match("some/**", "some/bar/baz/jquery.min.js"));

        assert!(glob_match("some/*/*/jquery.min.js", "some/bar/baz/jquery.min.js"));
        assert!(glob_match("some/**/jquery.min.js", "some/bar/baz/jquery.min.js"));
        assert!(glob_match("some/*/*/jquery.min.js", "some/bar/baz/jquery.min.js"));
        assert!(!glob_match("some/*/jquery.min.js", "some/bar/baz/jquery.min.js"));
        assert!(!glob_match("some/*/jquery.min.js", "some/bar/baz/jquery.min.js"));
    }

    #[test]
    fn basic() {
        assert!(glob_match("abc", "abc"));
        assert!(glob_match("*", "abc"));
        assert!(glob_match("*", ""));
        assert!(glob_match("**", ""));
        assert!(glob_match("*c", "abc"));
        assert!(!glob_match("*b", "abc"));
        assert!(glob_match("a*", "abc"));
        assert!(!glob_match("b*", "abc"));
        assert!(glob_match("a*", "a"));
        assert!(glob_match("*a", "a"));
        assert!(glob_match("a*b*c*d*e*", "axbxcxdxe"));
        assert!(glob_match("a*b*c*d*e*", "axbxcxdxexxx"));
        assert!(glob_match("a*b?c*x", "abxbbxdbxebxczzx"));
        assert!(!glob_match("a*b?c*x", "abxbbxdbxebxczzy"));

        assert!(glob_match("a/*/test", "a/foo/test"));
        assert!(!glob_match("a/*/test", "a/foo/bar/test"));
        assert!(glob_match("a/**/test", "a/foo/test"));
        assert!(glob_match("a/**/test", "a/foo/bar/test"));
        assert!(glob_match("a/**/b/c", "a/foo/bar/b/c"));
        assert!(glob_match("a\\*b", "a*b"));
        assert!(!glob_match("a\\*b", "axb"));

        assert!(glob_match("[abc]", "a"));
        assert!(glob_match("[abc]", "b"));
        assert!(glob_match("[abc]", "c"));
        assert!(!glob_match("[abc]", "d"));
        assert!(glob_match("x[abc]x", "xax"));
        assert!(glob_match("x[abc]x", "xbx"));
        assert!(glob_match("x[abc]x", "xcx"));
        assert!(!glob_match("x[abc]x", "xdx"));
        assert!(!glob_match("x[abc]x", "xay"));
        assert!(glob_match("[?]", "?"));
        assert!(!glob_match("[?]", "a"));
        assert!(glob_match("[*]", "*"));
        assert!(!glob_match("[*]", "a"));

        assert!(glob_match("[a-cx]", "a"));
        assert!(glob_match("[a-cx]", "b"));
        assert!(glob_match("[a-cx]", "c"));
        assert!(!glob_match("[a-cx]", "d"));
        assert!(glob_match("[a-cx]", "x"));

        assert!(!glob_match("[^abc]", "a"));
        assert!(!glob_match("[^abc]", "b"));
        assert!(!glob_match("[^abc]", "c"));
        assert!(glob_match("[^abc]", "d"));
        assert!(!glob_match("[!abc]", "a"));
        assert!(!glob_match("[!abc]", "b"));
        assert!(!glob_match("[!abc]", "c"));
        assert!(glob_match("[!abc]", "d"));
        assert!(glob_match("[\\!]", "!"));

        assert!(glob_match("a*b*[cy]*d*e*", "axbxcxdxexxx"));
        assert!(glob_match("a*b*[cy]*d*e*", "axbxyxdxexxx"));
        assert!(glob_match("a*b*[cy]*d*e*", "axbxxxyxdxexxx"));

        assert!(glob_match("test.{jpg,png}", "test.jpg"));
        assert!(glob_match("test.{jpg,png}", "test.png"));
        assert!(glob_match("test.{j*g,p*g}", "test.jpg"));
        assert!(glob_match("test.{j*g,p*g}", "test.jpxxxg"));
        assert!(glob_match("test.{j*g,p*g}", "test.jxg"));
        assert!(!glob_match("test.{j*g,p*g}", "test.jnt"));

        assert!(glob_match("test.{j*g,j*c}", "test.jnc"));
        assert!(glob_match("test.{jpg,p*g}", "test.png"));
        assert!(glob_match("test.{jpg,p*g}", "test.pxg"));
        assert!(!glob_match("test.{jpg,p*g}", "test.pnt"));
        assert!(glob_match("test.{jpeg,png}", "test.jpeg"));
        assert!(!glob_match("test.{jpeg,png}", "test.jpg"));
        assert!(glob_match("test.{jpeg,png}", "test.png"));
        assert!(glob_match("test.{jp\\,g,png}", "test.jp,g"));
        assert!(!glob_match("test.{jp\\,g,png}", "test.jxg"));
        assert!(glob_match("test/{foo,bar}/baz", "test/foo/baz"));
        assert!(glob_match("test/{foo,bar}/baz", "test/bar/baz"));
        assert!(!glob_match("test/{foo,bar}/baz", "test/baz/baz"));
        assert!(glob_match("test/{foo*,bar*}/baz", "test/foooooo/baz"));
        assert!(glob_match("test/{foo*,bar*}/baz", "test/barrrrr/baz"));
        assert!(glob_match("test/{*foo,*bar}/baz", "test/xxxxfoo/baz"));
        assert!(glob_match("test/{*foo,*bar}/baz", "test/xxxxbar/baz"));
        assert!(glob_match("test/{foo/**,bar}/baz", "test/bar/baz"));
        assert!(!glob_match("test/{foo/**,bar}/baz", "test/bar/test/baz"));

        assert!(!glob_match("*.txt", "some/big/path/to/the/needle.txt"));
        assert!(glob_match(
            "some/**/needle.{js,tsx,mdx,ts,jsx,txt}",
            "some/a/bigger/path/to/the/crazy/needle.txt"
        ));
        assert!(glob_match(
            "some/**/{a,b,c}/**/needle.txt",
            "some/foo/a/bigger/path/to/the/crazy/needle.txt"
        ));
        assert!(!glob_match(
            "some/**/{a,b,c}/**/needle.txt",
            "some/foo/d/bigger/path/to/the/crazy/needle.txt"
        ));

        assert!(glob_match("a/{a{a,b},b}", "a/aa"));
        assert!(glob_match("a/{a{a,b},b}", "a/ab"));
        assert!(!glob_match("a/{a{a,b},b}", "a/ac"));
        assert!(glob_match("a/{a{a,b},b}", "a/b"));
        assert!(!glob_match("a/{a{a,b},b}", "a/c"));
        assert!(glob_match("a/{b,c[}]*}", "a/b"));
        assert!(glob_match("a/{b,c[}]*}", "a/c}xx"));

        assert!(glob_match("/**/*a", "/a/a"));
        assert!(glob_match("**/*.js", "a/b.c/c.js"));
        assert!(glob_match("**/**/*.js", "a/b.c/c.js"));
        assert!(glob_match("a/**/*.d", "a/b/c.d"));
        assert!(glob_match("a/**/*.d", "a/.b/c.d"));

        assert!(glob_match("**/*/**", "a/b/c"));
        assert!(glob_match("**/*/c.js", "a/b/c.js"));
    }

    // The below tests are based on Bash and micromatch.
    // https://github.com/micromatch/picomatch/blob/master/test/bash.js
    // Converted using the following find and replace regex:
    // find: assert\(([!])?isMatch\('(.*?)', ['"](.*?)['"]\)\);
    // replace: assert!($1glob_match("$3", "$2"));

    #[test]
    fn bash() {
        assert!(!glob_match("a*", "*"));
        assert!(!glob_match("a*", "**"));
        assert!(!glob_match("a*", "\\*"));
        assert!(!glob_match("a*", "a/*"));
        assert!(!glob_match("a*", "b"));
        assert!(!glob_match("a*", "bc"));
        assert!(!glob_match("a*", "bcd"));
        assert!(!glob_match("a*", "bdir/"));
        assert!(!glob_match("a*", "Beware"));
        assert!(glob_match("a*", "a"));
        assert!(glob_match("a*", "ab"));
        assert!(glob_match("a*", "abc"));

        assert!(!glob_match("\\a*", "*"));
        assert!(!glob_match("\\a*", "**"));
        assert!(!glob_match("\\a*", "\\*"));

        assert!(glob_match("\\a*", "a"));
        assert!(!glob_match("\\a*", "a/*"));
        assert!(glob_match("\\a*", "abc"));
        assert!(glob_match("\\a*", "abd"));
        assert!(glob_match("\\a*", "abe"));
        assert!(!glob_match("\\a*", "b"));
        assert!(!glob_match("\\a*", "bb"));
        assert!(!glob_match("\\a*", "bcd"));
        assert!(!glob_match("\\a*", "bdir/"));
        assert!(!glob_match("\\a*", "Beware"));
        assert!(!glob_match("\\a*", "c"));
        assert!(!glob_match("\\a*", "ca"));
        assert!(!glob_match("\\a*", "cb"));
        assert!(!glob_match("\\a*", "d"));
        assert!(!glob_match("\\a*", "dd"));
        assert!(!glob_match("\\a*", "de"));
    }

    #[test]
    fn bash_directories() {
        assert!(!glob_match("b*/", "*"));
        assert!(!glob_match("b*/", "**"));
        assert!(!glob_match("b*/", "\\*"));
        assert!(!glob_match("b*/", "a"));
        assert!(!glob_match("b*/", "a/*"));
        assert!(!glob_match("b*/", "abc"));
        assert!(!glob_match("b*/", "abd"));
        assert!(!glob_match("b*/", "abe"));
        assert!(!glob_match("b*/", "b"));
        assert!(!glob_match("b*/", "bb"));
        assert!(!glob_match("b*/", "bcd"));
        assert!(glob_match("b*/", "bdir/"));
        assert!(!glob_match("b*/", "Beware"));
        assert!(!glob_match("b*/", "c"));
        assert!(!glob_match("b*/", "ca"));
        assert!(!glob_match("b*/", "cb"));
        assert!(!glob_match("b*/", "d"));
        assert!(!glob_match("b*/", "dd"));
        assert!(!glob_match("b*/", "de"));
    }

    #[test]
    fn bash_escaping() {
        assert!(!glob_match("\\^", "*"));
        assert!(!glob_match("\\^", "**"));
        assert!(!glob_match("\\^", "\\*"));
        assert!(!glob_match("\\^", "a"));
        assert!(!glob_match("\\^", "a/*"));
        assert!(!glob_match("\\^", "abc"));
        assert!(!glob_match("\\^", "abd"));
        assert!(!glob_match("\\^", "abe"));
        assert!(!glob_match("\\^", "b"));
        assert!(!glob_match("\\^", "bb"));
        assert!(!glob_match("\\^", "bcd"));
        assert!(!glob_match("\\^", "bdir/"));
        assert!(!glob_match("\\^", "Beware"));
        assert!(!glob_match("\\^", "c"));
        assert!(!glob_match("\\^", "ca"));
        assert!(!glob_match("\\^", "cb"));
        assert!(!glob_match("\\^", "d"));
        assert!(!glob_match("\\^", "dd"));
        assert!(!glob_match("\\^", "de"));

        assert!(glob_match("\\*", "*"));
        // assert!(glob_match("\\*", "\\*"));
        assert!(!glob_match("\\*", "**"));
        assert!(!glob_match("\\*", "a"));
        assert!(!glob_match("\\*", "a/*"));
        assert!(!glob_match("\\*", "abc"));
        assert!(!glob_match("\\*", "abd"));
        assert!(!glob_match("\\*", "abe"));
        assert!(!glob_match("\\*", "b"));
        assert!(!glob_match("\\*", "bb"));
        assert!(!glob_match("\\*", "bcd"));
        assert!(!glob_match("\\*", "bdir/"));
        assert!(!glob_match("\\*", "Beware"));
        assert!(!glob_match("\\*", "c"));
        assert!(!glob_match("\\*", "ca"));
        assert!(!glob_match("\\*", "cb"));
        assert!(!glob_match("\\*", "d"));
        assert!(!glob_match("\\*", "dd"));
        assert!(!glob_match("\\*", "de"));

        assert!(!glob_match("a\\*", "*"));
        assert!(!glob_match("a\\*", "**"));
        assert!(!glob_match("a\\*", "\\*"));
        assert!(!glob_match("a\\*", "a"));
        assert!(!glob_match("a\\*", "a/*"));
        assert!(!glob_match("a\\*", "abc"));
        assert!(!glob_match("a\\*", "abd"));
        assert!(!glob_match("a\\*", "abe"));
        assert!(!glob_match("a\\*", "b"));
        assert!(!glob_match("a\\*", "bb"));
        assert!(!glob_match("a\\*", "bcd"));
        assert!(!glob_match("a\\*", "bdir/"));
        assert!(!glob_match("a\\*", "Beware"));
        assert!(!glob_match("a\\*", "c"));
        assert!(!glob_match("a\\*", "ca"));
        assert!(!glob_match("a\\*", "cb"));
        assert!(!glob_match("a\\*", "d"));
        assert!(!glob_match("a\\*", "dd"));
        assert!(!glob_match("a\\*", "de"));

        assert!(glob_match("*q*", "aqa"));
        assert!(glob_match("*q*", "aaqaa"));
        assert!(!glob_match("*q*", "*"));
        assert!(!glob_match("*q*", "**"));
        assert!(!glob_match("*q*", "\\*"));
        assert!(!glob_match("*q*", "a"));
        assert!(!glob_match("*q*", "a/*"));
        assert!(!glob_match("*q*", "abc"));
        assert!(!glob_match("*q*", "abd"));
        assert!(!glob_match("*q*", "abe"));
        assert!(!glob_match("*q*", "b"));
        assert!(!glob_match("*q*", "bb"));
        assert!(!glob_match("*q*", "bcd"));
        assert!(!glob_match("*q*", "bdir/"));
        assert!(!glob_match("*q*", "Beware"));
        assert!(!glob_match("*q*", "c"));
        assert!(!glob_match("*q*", "ca"));
        assert!(!glob_match("*q*", "cb"));
        assert!(!glob_match("*q*", "d"));
        assert!(!glob_match("*q*", "dd"));
        assert!(!glob_match("*q*", "de"));

        assert!(glob_match("\\**", "*"));
        assert!(glob_match("\\**", "**"));
        assert!(!glob_match("\\**", "\\*"));
        assert!(!glob_match("\\**", "a"));
        assert!(!glob_match("\\**", "a/*"));
        assert!(!glob_match("\\**", "abc"));
        assert!(!glob_match("\\**", "abd"));
        assert!(!glob_match("\\**", "abe"));
        assert!(!glob_match("\\**", "b"));
        assert!(!glob_match("\\**", "bb"));
        assert!(!glob_match("\\**", "bcd"));
        assert!(!glob_match("\\**", "bdir/"));
        assert!(!glob_match("\\**", "Beware"));
        assert!(!glob_match("\\**", "c"));
        assert!(!glob_match("\\**", "ca"));
        assert!(!glob_match("\\**", "cb"));
        assert!(!glob_match("\\**", "d"));
        assert!(!glob_match("\\**", "dd"));
        assert!(!glob_match("\\**", "de"));
    }

    #[test]
    fn bash_classes() {
        assert!(!glob_match("a*[^c]", "*"));
        assert!(!glob_match("a*[^c]", "**"));
        assert!(!glob_match("a*[^c]", "\\*"));
        assert!(!glob_match("a*[^c]", "a"));
        assert!(!glob_match("a*[^c]", "a/*"));
        assert!(!glob_match("a*[^c]", "abc"));
        assert!(glob_match("a*[^c]", "abd"));
        assert!(glob_match("a*[^c]", "abe"));
        assert!(!glob_match("a*[^c]", "b"));
        assert!(!glob_match("a*[^c]", "bb"));
        assert!(!glob_match("a*[^c]", "bcd"));
        assert!(!glob_match("a*[^c]", "bdir/"));
        assert!(!glob_match("a*[^c]", "Beware"));
        assert!(!glob_match("a*[^c]", "c"));
        assert!(!glob_match("a*[^c]", "ca"));
        assert!(!glob_match("a*[^c]", "cb"));
        assert!(!glob_match("a*[^c]", "d"));
        assert!(!glob_match("a*[^c]", "dd"));
        assert!(!glob_match("a*[^c]", "de"));
        assert!(!glob_match("a*[^c]", "baz"));
        assert!(!glob_match("a*[^c]", "bzz"));
        assert!(!glob_match("a*[^c]", "BZZ"));
        assert!(!glob_match("a*[^c]", "beware"));
        assert!(!glob_match("a*[^c]", "BewAre"));

        assert!(glob_match("a[X-]b", "a-b"));
        assert!(glob_match("a[X-]b", "aXb"));

        assert!(!glob_match("[a-y]*[^c]", "*"));
        assert!(glob_match("[a-y]*[^c]", "a*"));
        assert!(!glob_match("[a-y]*[^c]", "**"));
        assert!(!glob_match("[a-y]*[^c]", "\\*"));
        assert!(!glob_match("[a-y]*[^c]", "a"));
        assert!(glob_match("[a-y]*[^c]", "a123b"));
        assert!(!glob_match("[a-y]*[^c]", "a123c"));
        assert!(glob_match("[a-y]*[^c]", "ab"));
        assert!(!glob_match("[a-y]*[^c]", "a/*"));
        assert!(!glob_match("[a-y]*[^c]", "abc"));
        assert!(glob_match("[a-y]*[^c]", "abd"));
        assert!(glob_match("[a-y]*[^c]", "abe"));
        assert!(!glob_match("[a-y]*[^c]", "b"));
        assert!(glob_match("[a-y]*[^c]", "bd"));
        assert!(glob_match("[a-y]*[^c]", "bb"));
        assert!(glob_match("[a-y]*[^c]", "bcd"));
        assert!(glob_match("[a-y]*[^c]", "bdir/"));
        assert!(!glob_match("[a-y]*[^c]", "Beware"));
        assert!(!glob_match("[a-y]*[^c]", "c"));
        assert!(glob_match("[a-y]*[^c]", "ca"));
        assert!(glob_match("[a-y]*[^c]", "cb"));
        assert!(!glob_match("[a-y]*[^c]", "d"));
        assert!(glob_match("[a-y]*[^c]", "dd"));
        assert!(glob_match("[a-y]*[^c]", "de"));
        assert!(glob_match("[a-y]*[^c]", "baz"));
        assert!(glob_match("[a-y]*[^c]", "bzz"));
        // assert(!isMatch('bzz', '[a-y]*[^c]', { regex: true }));
        assert!(!glob_match("[a-y]*[^c]", "BZZ"));
        assert!(glob_match("[a-y]*[^c]", "beware"));
        assert!(!glob_match("[a-y]*[^c]", "BewAre"));

        assert!(glob_match("a\\*b/*", "a*b/ooo"));
        assert!(glob_match("a\\*?/*", "a*b/ooo"));

        assert!(!glob_match("a[b]c", "*"));
        assert!(!glob_match("a[b]c", "**"));
        assert!(!glob_match("a[b]c", "\\*"));
        assert!(!glob_match("a[b]c", "a"));
        assert!(!glob_match("a[b]c", "a/*"));
        assert!(glob_match("a[b]c", "abc"));
        assert!(!glob_match("a[b]c", "abd"));
        assert!(!glob_match("a[b]c", "abe"));
        assert!(!glob_match("a[b]c", "b"));
        assert!(!glob_match("a[b]c", "bb"));
        assert!(!glob_match("a[b]c", "bcd"));
        assert!(!glob_match("a[b]c", "bdir/"));
        assert!(!glob_match("a[b]c", "Beware"));
        assert!(!glob_match("a[b]c", "c"));
        assert!(!glob_match("a[b]c", "ca"));
        assert!(!glob_match("a[b]c", "cb"));
        assert!(!glob_match("a[b]c", "d"));
        assert!(!glob_match("a[b]c", "dd"));
        assert!(!glob_match("a[b]c", "de"));
        assert!(!glob_match("a[b]c", "baz"));
        assert!(!glob_match("a[b]c", "bzz"));
        assert!(!glob_match("a[b]c", "BZZ"));
        assert!(!glob_match("a[b]c", "beware"));
        assert!(!glob_match("a[b]c", "BewAre"));

        assert!(!glob_match("a[\"b\"]c", "*"));
        assert!(!glob_match("a[\"b\"]c", "**"));
        assert!(!glob_match("a[\"b\"]c", "\\*"));
        assert!(!glob_match("a[\"b\"]c", "a"));
        assert!(!glob_match("a[\"b\"]c", "a/*"));
        assert!(glob_match("a[\"b\"]c", "abc"));
        assert!(!glob_match("a[\"b\"]c", "abd"));
        assert!(!glob_match("a[\"b\"]c", "abe"));
        assert!(!glob_match("a[\"b\"]c", "b"));
        assert!(!glob_match("a[\"b\"]c", "bb"));
        assert!(!glob_match("a[\"b\"]c", "bcd"));
        assert!(!glob_match("a[\"b\"]c", "bdir/"));
        assert!(!glob_match("a[\"b\"]c", "Beware"));
        assert!(!glob_match("a[\"b\"]c", "c"));
        assert!(!glob_match("a[\"b\"]c", "ca"));
        assert!(!glob_match("a[\"b\"]c", "cb"));
        assert!(!glob_match("a[\"b\"]c", "d"));
        assert!(!glob_match("a[\"b\"]c", "dd"));
        assert!(!glob_match("a[\"b\"]c", "de"));
        assert!(!glob_match("a[\"b\"]c", "baz"));
        assert!(!glob_match("a[\"b\"]c", "bzz"));
        assert!(!glob_match("a[\"b\"]c", "BZZ"));
        assert!(!glob_match("a[\"b\"]c", "beware"));
        assert!(!glob_match("a[\"b\"]c", "BewAre"));

        assert!(!glob_match("a[\\\\b]c", "*"));
        assert!(!glob_match("a[\\\\b]c", "**"));
        assert!(!glob_match("a[\\\\b]c", "\\*"));
        assert!(!glob_match("a[\\\\b]c", "a"));
        assert!(!glob_match("a[\\\\b]c", "a/*"));
        assert!(glob_match("a[\\\\b]c", "abc"));
        assert!(!glob_match("a[\\\\b]c", "abd"));
        assert!(!glob_match("a[\\\\b]c", "abe"));
        assert!(!glob_match("a[\\\\b]c", "b"));
        assert!(!glob_match("a[\\\\b]c", "bb"));
        assert!(!glob_match("a[\\\\b]c", "bcd"));
        assert!(!glob_match("a[\\\\b]c", "bdir/"));
        assert!(!glob_match("a[\\\\b]c", "Beware"));
        assert!(!glob_match("a[\\\\b]c", "c"));
        assert!(!glob_match("a[\\\\b]c", "ca"));
        assert!(!glob_match("a[\\\\b]c", "cb"));
        assert!(!glob_match("a[\\\\b]c", "d"));
        assert!(!glob_match("a[\\\\b]c", "dd"));
        assert!(!glob_match("a[\\\\b]c", "de"));
        assert!(!glob_match("a[\\\\b]c", "baz"));
        assert!(!glob_match("a[\\\\b]c", "bzz"));
        assert!(!glob_match("a[\\\\b]c", "BZZ"));
        assert!(!glob_match("a[\\\\b]c", "beware"));
        assert!(!glob_match("a[\\\\b]c", "BewAre"));

        assert!(!glob_match("a[\\b]c", "*"));
        assert!(!glob_match("a[\\b]c", "**"));
        assert!(!glob_match("a[\\b]c", "\\*"));
        assert!(!glob_match("a[\\b]c", "a"));
        assert!(!glob_match("a[\\b]c", "a/*"));
        assert!(!glob_match("a[\\b]c", "abc"));
        assert!(!glob_match("a[\\b]c", "abd"));
        assert!(!glob_match("a[\\b]c", "abe"));
        assert!(!glob_match("a[\\b]c", "b"));
        assert!(!glob_match("a[\\b]c", "bb"));
        assert!(!glob_match("a[\\b]c", "bcd"));
        assert!(!glob_match("a[\\b]c", "bdir/"));
        assert!(!glob_match("a[\\b]c", "Beware"));
        assert!(!glob_match("a[\\b]c", "c"));
        assert!(!glob_match("a[\\b]c", "ca"));
        assert!(!glob_match("a[\\b]c", "cb"));
        assert!(!glob_match("a[\\b]c", "d"));
        assert!(!glob_match("a[\\b]c", "dd"));
        assert!(!glob_match("a[\\b]c", "de"));
        assert!(!glob_match("a[\\b]c", "baz"));
        assert!(!glob_match("a[\\b]c", "bzz"));
        assert!(!glob_match("a[\\b]c", "BZZ"));
        assert!(!glob_match("a[\\b]c", "beware"));
        assert!(!glob_match("a[\\b]c", "BewAre"));

        assert!(!glob_match("a[b-d]c", "*"));
        assert!(!glob_match("a[b-d]c", "**"));
        assert!(!glob_match("a[b-d]c", "\\*"));
        assert!(!glob_match("a[b-d]c", "a"));
        assert!(!glob_match("a[b-d]c", "a/*"));
        assert!(glob_match("a[b-d]c", "abc"));
        assert!(!glob_match("a[b-d]c", "abd"));
        assert!(!glob_match("a[b-d]c", "abe"));
        assert!(!glob_match("a[b-d]c", "b"));
        assert!(!glob_match("a[b-d]c", "bb"));
        assert!(!glob_match("a[b-d]c", "bcd"));
        assert!(!glob_match("a[b-d]c", "bdir/"));
        assert!(!glob_match("a[b-d]c", "Beware"));
        assert!(!glob_match("a[b-d]c", "c"));
        assert!(!glob_match("a[b-d]c", "ca"));
        assert!(!glob_match("a[b-d]c", "cb"));
        assert!(!glob_match("a[b-d]c", "d"));
        assert!(!glob_match("a[b-d]c", "dd"));
        assert!(!glob_match("a[b-d]c", "de"));
        assert!(!glob_match("a[b-d]c", "baz"));
        assert!(!glob_match("a[b-d]c", "bzz"));
        assert!(!glob_match("a[b-d]c", "BZZ"));
        assert!(!glob_match("a[b-d]c", "beware"));
        assert!(!glob_match("a[b-d]c", "BewAre"));

        assert!(!glob_match("a?c", "*"));
        assert!(!glob_match("a?c", "**"));
        assert!(!glob_match("a?c", "\\*"));
        assert!(!glob_match("a?c", "a"));
        assert!(!glob_match("a?c", "a/*"));
        assert!(glob_match("a?c", "abc"));
        assert!(!glob_match("a?c", "abd"));
        assert!(!glob_match("a?c", "abe"));
        assert!(!glob_match("a?c", "b"));
        assert!(!glob_match("a?c", "bb"));
        assert!(!glob_match("a?c", "bcd"));
        assert!(!glob_match("a?c", "bdir/"));
        assert!(!glob_match("a?c", "Beware"));
        assert!(!glob_match("a?c", "c"));
        assert!(!glob_match("a?c", "ca"));
        assert!(!glob_match("a?c", "cb"));
        assert!(!glob_match("a?c", "d"));
        assert!(!glob_match("a?c", "dd"));
        assert!(!glob_match("a?c", "de"));
        assert!(!glob_match("a?c", "baz"));
        assert!(!glob_match("a?c", "bzz"));
        assert!(!glob_match("a?c", "BZZ"));
        assert!(!glob_match("a?c", "beware"));
        assert!(!glob_match("a?c", "BewAre"));

        assert!(glob_match("*/man*/bash.*", "man/man1/bash.1"));

        assert!(glob_match("[^a-c]*", "*"));
        assert!(glob_match("[^a-c]*", "**"));
        assert!(!glob_match("[^a-c]*", "a"));
        assert!(!glob_match("[^a-c]*", "a/*"));
        assert!(!glob_match("[^a-c]*", "abc"));
        assert!(!glob_match("[^a-c]*", "abd"));
        assert!(!glob_match("[^a-c]*", "abe"));
        assert!(!glob_match("[^a-c]*", "b"));
        assert!(!glob_match("[^a-c]*", "bb"));
        assert!(!glob_match("[^a-c]*", "bcd"));
        assert!(!glob_match("[^a-c]*", "bdir/"));
        assert!(glob_match("[^a-c]*", "Beware"));
        assert!(!glob_match("[^a-c]*", "c"));
        assert!(!glob_match("[^a-c]*", "ca"));
        assert!(!glob_match("[^a-c]*", "cb"));
        assert!(glob_match("[^a-c]*", "d"));
        assert!(glob_match("[^a-c]*", "dd"));
        assert!(glob_match("[^a-c]*", "de"));
        assert!(!glob_match("[^a-c]*", "baz"));
        assert!(!glob_match("[^a-c]*", "bzz"));
        assert!(glob_match("[^a-c]*", "BZZ"));
        assert!(!glob_match("[^a-c]*", "beware"));
        assert!(glob_match("[^a-c]*", "BewAre"));
    }

    #[test]
    fn bash_wildmatch() {
        assert!(!glob_match("a[]-]b", "aab"));
        assert!(!glob_match("[ten]", "ten"));
        assert!(glob_match("]", "]"));
        assert!(glob_match("a[]-]b", "a-b"));
        assert!(glob_match("a[]-]b", "a]b"));
        assert!(glob_match("a[]]b", "a]b"));
        assert!(glob_match("a[\\]a\\-]b", "aab"));
        assert!(glob_match("t[a-g]n", "ten"));
        assert!(glob_match("t[^a-g]n", "ton"));
    }

    #[test]
    fn bash_slashmatch() {
        // assert!(!glob_match("f[^eiu][^eiu][^eiu][^eiu][^eiu]r", "foo/bar"));
        assert!(glob_match("foo[/]bar", "foo/bar"));
        assert!(glob_match("f[^eiu][^eiu][^eiu][^eiu][^eiu]r", "foo-bar"));
    }

    #[test]
    fn bash_extra_stars() {
        assert!(!glob_match("a**c", "bbc"));
        assert!(glob_match("a**c", "abc"));
        assert!(!glob_match("a**c", "bbd"));

        assert!(!glob_match("a***c", "bbc"));
        assert!(glob_match("a***c", "abc"));
        assert!(!glob_match("a***c", "bbd"));

        assert!(!glob_match("a*****?c", "bbc"));
        assert!(glob_match("a*****?c", "abc"));
        assert!(!glob_match("a*****?c", "bbc"));

        assert!(glob_match("?*****??", "bbc"));
        assert!(glob_match("?*****??", "abc"));

        assert!(glob_match("*****??", "bbc"));
        assert!(glob_match("*****??", "abc"));

        assert!(glob_match("?*****?c", "bbc"));
        assert!(glob_match("?*****?c", "abc"));

        assert!(glob_match("?***?****c", "bbc"));
        assert!(glob_match("?***?****c", "abc"));
        assert!(!glob_match("?***?****c", "bbd"));

        assert!(glob_match("?***?****?", "bbc"));
        assert!(glob_match("?***?****?", "abc"));

        assert!(glob_match("?***?****", "bbc"));
        assert!(glob_match("?***?****", "abc"));

        assert!(glob_match("*******c", "bbc"));
        assert!(glob_match("*******c", "abc"));

        assert!(glob_match("*******?", "bbc"));
        assert!(glob_match("*******?", "abc"));

        assert!(glob_match("a*cd**?**??k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??k***", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??***k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??***k**", "abcdecdhjk"));
        assert!(glob_match("a****c**?**??*****", "abcdecdhjk"));
    }

    #[test]
    fn stars() {
        assert!(!glob_match("*.js", "a/b/c/z.js"));
        assert!(!glob_match("*.js", "a/b/z.js"));
        assert!(!glob_match("*.js", "a/z.js"));
        assert!(glob_match("*.js", "z.js"));

        // assert!(!glob_match("*/*", "a/.ab"));
        // assert!(!glob_match("*", ".ab"));

        assert!(glob_match("z*.js", "z.js"));
        assert!(glob_match("*/*", "a/z"));
        assert!(glob_match("*/z*.js", "a/z.js"));
        assert!(glob_match("a/z*.js", "a/z.js"));

        assert!(glob_match("*", "ab"));
        assert!(glob_match("*", "abc"));

        assert!(!glob_match("f*", "bar"));
        assert!(!glob_match("*r", "foo"));
        assert!(!glob_match("b*", "foo"));
        assert!(!glob_match("*", "foo/bar"));
        assert!(glob_match("*c", "abc"));
        assert!(glob_match("a*", "abc"));
        assert!(glob_match("a*c", "abc"));
        assert!(glob_match("*r", "bar"));
        assert!(glob_match("b*", "bar"));
        assert!(glob_match("f*", "foo"));

        assert!(glob_match("*abc*", "one abc two"));
        assert!(glob_match("a*b", "a         b"));

        assert!(!glob_match("*a*", "foo"));
        assert!(glob_match("*a*", "bar"));
        assert!(glob_match("*abc*", "oneabctwo"));
        assert!(!glob_match("*-bc-*", "a-b.c-d"));
        assert!(glob_match("*-*.*-*", "a-b.c-d"));
        assert!(glob_match("*-b*c-*", "a-b.c-d"));
        assert!(glob_match("*-b.c-*", "a-b.c-d"));
        assert!(glob_match("*.*", "a-b.c-d"));
        assert!(glob_match("*.*-*", "a-b.c-d"));
        assert!(glob_match("*.*-d", "a-b.c-d"));
        assert!(glob_match("*.c-*", "a-b.c-d"));
        assert!(glob_match("*b.*d", "a-b.c-d"));
        assert!(glob_match("a*.c*", "a-b.c-d"));
        assert!(glob_match("a-*.*-d", "a-b.c-d"));
        assert!(glob_match("*.*", "a.b"));
        assert!(glob_match("*.b", "a.b"));
        assert!(glob_match("a.*", "a.b"));
        assert!(glob_match("a.b", "a.b"));

        assert!(!glob_match("**-bc-**", "a-b.c-d"));
        assert!(glob_match("**-**.**-**", "a-b.c-d"));
        assert!(glob_match("**-b**c-**", "a-b.c-d"));
        assert!(glob_match("**-b.c-**", "a-b.c-d"));
        assert!(glob_match("**.**", "a-b.c-d"));
        assert!(glob_match("**.**-**", "a-b.c-d"));
        assert!(glob_match("**.**-d", "a-b.c-d"));
        assert!(glob_match("**.c-**", "a-b.c-d"));
        assert!(glob_match("**b.**d", "a-b.c-d"));
        assert!(glob_match("a**.c**", "a-b.c-d"));
        assert!(glob_match("a-**.**-d", "a-b.c-d"));
        assert!(glob_match("**.**", "a.b"));
        assert!(glob_match("**.b", "a.b"));
        assert!(glob_match("a.**", "a.b"));
        assert!(glob_match("a.b", "a.b"));

        assert!(glob_match("*/*", "/ab"));
        assert!(glob_match(".", "."));
        assert!(!glob_match("a/", "a/.b"));
        assert!(glob_match("/*", "/ab"));
        assert!(glob_match("/??", "/ab"));
        assert!(glob_match("/?b", "/ab"));
        assert!(glob_match("/*", "/cd"));
        assert!(glob_match("a", "a"));
        assert!(glob_match("a/.*", "a/.b"));
        assert!(glob_match("?/?", "a/b"));
        assert!(glob_match("a/**/j/**/z/*.md", "a/b/c/d/e/j/n/p/o/z/c.md"));
        assert!(glob_match("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
        assert!(glob_match("a/b/c/*.md", "a/b/c/xyz.md"));
        assert!(glob_match("a/*/z/.a", "a/b/z/.a"));
        assert!(!glob_match("bz", "a/b/z/.a"));
        assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/b.b/aa/c/xyz.md"));
        assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/bb/aa/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bbbb/c/xyz.md"));
        assert!(glob_match("*", "aaa"));
        assert!(glob_match("*", "ab"));
        assert!(glob_match("ab", "ab"));

        assert!(!glob_match("*/*/*", "aaa"));
        assert!(!glob_match("*/*/*", "aaa/bb/aa/rr"));
        assert!(!glob_match("aaa*", "aaa/bba/ccc"));
        // assert!(!glob_match("aaa**", "aaa/bba/ccc"));
        assert!(!glob_match("aaa/*", "aaa/bba/ccc"));
        assert!(!glob_match("aaa/*ccc", "aaa/bba/ccc"));
        assert!(!glob_match("aaa/*z", "aaa/bba/ccc"));
        assert!(!glob_match("*/*/*", "aaa/bbb"));
        assert!(!glob_match("*/*jk*/*i", "ab/zzz/ejkl/hi"));
        assert!(glob_match("*/*/*", "aaa/bba/ccc"));
        assert!(glob_match("aaa/**", "aaa/bba/ccc"));
        assert!(glob_match("aaa/*", "aaa/bbb"));
        assert!(glob_match("*/*z*/*/*i", "ab/zzz/ejkl/hi"));
        assert!(glob_match("*j*i", "abzzzejklhi"));

        assert!(glob_match("*", "a"));
        assert!(glob_match("*", "b"));
        assert!(!glob_match("*", "a/a"));
        assert!(!glob_match("*", "a/a/a"));
        assert!(!glob_match("*", "a/a/b"));
        assert!(!glob_match("*", "a/a/a/a"));
        assert!(!glob_match("*", "a/a/a/a/a"));

        assert!(!glob_match("*/*", "a"));
        assert!(glob_match("*/*", "a/a"));
        assert!(!glob_match("*/*", "a/a/a"));

        assert!(!glob_match("*/*/*", "a"));
        assert!(!glob_match("*/*/*", "a/a"));
        assert!(glob_match("*/*/*", "a/a/a"));
        assert!(!glob_match("*/*/*", "a/a/a/a"));

        assert!(!glob_match("*/*/*/*", "a"));
        assert!(!glob_match("*/*/*/*", "a/a"));
        assert!(!glob_match("*/*/*/*", "a/a/a"));
        assert!(glob_match("*/*/*/*", "a/a/a/a"));
        assert!(!glob_match("*/*/*/*", "a/a/a/a/a"));

        assert!(!glob_match("*/*/*/*/*", "a"));
        assert!(!glob_match("*/*/*/*/*", "a/a"));
        assert!(!glob_match("*/*/*/*/*", "a/a/a"));
        assert!(!glob_match("*/*/*/*/*", "a/a/b"));
        assert!(!glob_match("*/*/*/*/*", "a/a/a/a"));
        assert!(glob_match("*/*/*/*/*", "a/a/a/a/a"));
        assert!(!glob_match("*/*/*/*/*", "a/a/a/a/a/a"));

        assert!(!glob_match("a/*", "a"));
        assert!(glob_match("a/*", "a/a"));
        assert!(!glob_match("a/*", "a/a/a"));
        assert!(!glob_match("a/*", "a/a/a/a"));
        assert!(!glob_match("a/*", "a/a/a/a/a"));

        assert!(!glob_match("a/*/*", "a"));
        assert!(!glob_match("a/*/*", "a/a"));
        assert!(glob_match("a/*/*", "a/a/a"));
        assert!(!glob_match("a/*/*", "b/a/a"));
        assert!(!glob_match("a/*/*", "a/a/a/a"));
        assert!(!glob_match("a/*/*", "a/a/a/a/a"));

        assert!(!glob_match("a/*/*/*", "a"));
        assert!(!glob_match("a/*/*/*", "a/a"));
        assert!(!glob_match("a/*/*/*", "a/a/a"));
        assert!(glob_match("a/*/*/*", "a/a/a/a"));
        assert!(!glob_match("a/*/*/*", "a/a/a/a/a"));

        assert!(!glob_match("a/*/*/*/*", "a"));
        assert!(!glob_match("a/*/*/*/*", "a/a"));
        assert!(!glob_match("a/*/*/*/*", "a/a/a"));
        assert!(!glob_match("a/*/*/*/*", "a/a/b"));
        assert!(!glob_match("a/*/*/*/*", "a/a/a/a"));
        assert!(glob_match("a/*/*/*/*", "a/a/a/a/a"));

        assert!(!glob_match("a/*/a", "a"));
        assert!(!glob_match("a/*/a", "a/a"));
        assert!(glob_match("a/*/a", "a/a/a"));
        assert!(!glob_match("a/*/a", "a/a/b"));
        assert!(!glob_match("a/*/a", "a/a/a/a"));
        assert!(!glob_match("a/*/a", "a/a/a/a/a"));

        assert!(!glob_match("a/*/b", "a"));
        assert!(!glob_match("a/*/b", "a/a"));
        assert!(!glob_match("a/*/b", "a/a/a"));
        assert!(glob_match("a/*/b", "a/a/b"));
        assert!(!glob_match("a/*/b", "a/a/a/a"));
        assert!(!glob_match("a/*/b", "a/a/a/a/a"));

        assert!(!glob_match("*/**/a", "a"));
        assert!(!glob_match("*/**/a", "a/a/b"));
        assert!(glob_match("*/**/a", "a/a"));
        assert!(glob_match("*/**/a", "a/a/a"));
        assert!(glob_match("*/**/a", "a/a/a/a"));
        assert!(glob_match("*/**/a", "a/a/a/a/a"));

        assert!(!glob_match("*/", "a"));
        assert!(!glob_match("*/*", "a"));
        assert!(!glob_match("a/*", "a"));
        // assert!(!glob_match("*/*", "a/"));
        // assert!(!glob_match("a/*", "a/"));
        assert!(!glob_match("*", "a/a"));
        assert!(!glob_match("*/", "a/a"));
        assert!(!glob_match("*/", "a/x/y"));
        assert!(!glob_match("*/*", "a/x/y"));
        assert!(!glob_match("a/*", "a/x/y"));
        // assert!(glob_match("*", "a/"));
        assert!(glob_match("*", "a"));
        assert!(glob_match("*/", "a/"));
        assert!(glob_match("*{,/}", "a/"));
        assert!(glob_match("*/*", "a/a"));
        assert!(glob_match("a/*", "a/a"));

        assert!(!glob_match("a/**/*.txt", "a.txt"));
        assert!(glob_match("a/**/*.txt", "a/x/y.txt"));
        assert!(!glob_match("a/**/*.txt", "a/x/y/z"));

        assert!(!glob_match("a/*.txt", "a.txt"));
        assert!(glob_match("a/*.txt", "a/b.txt"));
        assert!(!glob_match("a/*.txt", "a/x/y.txt"));
        assert!(!glob_match("a/*.txt", "a/x/y/z"));

        assert!(glob_match("a*.txt", "a.txt"));
        assert!(!glob_match("a*.txt", "a/b.txt"));
        assert!(!glob_match("a*.txt", "a/x/y.txt"));
        assert!(!glob_match("a*.txt", "a/x/y/z"));

        assert!(glob_match("*.txt", "a.txt"));
        assert!(!glob_match("*.txt", "a/b.txt"));
        assert!(!glob_match("*.txt", "a/x/y.txt"));
        assert!(!glob_match("*.txt", "a/x/y/z"));

        assert!(!glob_match("a*", "a/b"));
        assert!(!glob_match("a/**/b", "a/a/bb"));
        assert!(!glob_match("a/**/b", "a/bb"));

        assert!(!glob_match("*/**", "foo"));
        assert!(!glob_match("**/", "foo/bar"));
        assert!(!glob_match("**/*/", "foo/bar"));
        assert!(!glob_match("*/*/", "foo/bar"));

        assert!(glob_match("**/..", "/home/foo/.."));
        assert!(glob_match("**/a", "a"));
        assert!(glob_match("**", "a/a"));
        assert!(glob_match("a/**", "a/a"));
        assert!(glob_match("a/**", "a/"));
        // assert!(glob_match("a/**", "a"));
        assert!(!glob_match("**/", "a/a"));
        // assert!(glob_match("**/a/**", "a"));
        // assert!(glob_match("a/**", "a"));
        assert!(!glob_match("**/", "a/a"));
        assert!(glob_match("*/**/a", "a/a"));
        // assert!(glob_match("a/**", "a"));
        assert!(glob_match("*/**", "foo/"));
        assert!(glob_match("**/*", "foo/bar"));
        assert!(glob_match("*/*", "foo/bar"));
        assert!(glob_match("*/**", "foo/bar"));
        assert!(glob_match("**/", "foo/bar/"));
        // assert!(glob_match("**/*", "foo/bar/"));
        assert!(glob_match("**/*/", "foo/bar/"));
        assert!(glob_match("*/**", "foo/bar/"));
        assert!(glob_match("*/*/", "foo/bar/"));

        assert!(!glob_match("*/foo", "bar/baz/foo"));
        assert!(!glob_match("**/bar/*", "deep/foo/bar"));
        assert!(!glob_match("*/bar/**", "deep/foo/bar/baz/x"));
        assert!(!glob_match("/*", "ef"));
        assert!(!glob_match("foo?bar", "foo/bar"));
        assert!(!glob_match("**/bar*", "foo/bar/baz"));
        // assert!(!glob_match("**/bar**", "foo/bar/baz"));
        assert!(!glob_match("foo**bar", "foo/baz/bar"));
        assert!(!glob_match("foo*bar", "foo/baz/bar"));
        // assert!(glob_match("foo/**", "foo"));
        assert!(glob_match("/*", "/ab"));
        assert!(glob_match("/*", "/cd"));
        assert!(glob_match("/*", "/ef"));
        assert!(glob_match("a/**/j/**/z/*.md", "a/b/j/c/z/x.md"));
        assert!(glob_match("a/**/j/**/z/*.md", "a/j/z/x.md"));

        assert!(glob_match("**/foo", "bar/baz/foo"));
        assert!(glob_match("**/bar/*", "deep/foo/bar/baz"));
        assert!(glob_match("**/bar/**", "deep/foo/bar/baz/"));
        assert!(glob_match("**/bar/*/*", "deep/foo/bar/baz/x"));
        assert!(glob_match("foo/**/**/bar", "foo/b/a/z/bar"));
        assert!(glob_match("foo/**/bar", "foo/b/a/z/bar"));
        assert!(glob_match("foo/**/**/bar", "foo/bar"));
        assert!(glob_match("foo/**/bar", "foo/bar"));
        assert!(glob_match("*/bar/**", "foo/bar/baz/x"));
        assert!(glob_match("foo/**/**/bar", "foo/baz/bar"));
        assert!(glob_match("foo/**/bar", "foo/baz/bar"));
        assert!(glob_match("**/foo", "XXX/foo"));
    }

    #[test]
    fn globstars() {
        assert!(glob_match("**/*.js", "a/b/c/d.js"));
        assert!(glob_match("**/*.js", "a/b/c.js"));
        assert!(glob_match("**/*.js", "a/b.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/c/d/e/f.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/c/d/e.js"));
        assert!(glob_match("a/b/c/**/*.js", "a/b/c/d.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/c/d.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/d.js"));
        assert!(!glob_match("a/b/**/*.js", "a/d.js"));
        assert!(!glob_match("a/b/**/*.js", "d.js"));

        assert!(!glob_match("**c", "a/b/c"));
        assert!(!glob_match("a/**c", "a/b/c"));
        assert!(!glob_match("a/**z", "a/b/c"));
        assert!(!glob_match("a/**b**/c", "a/b/c/b/c"));
        assert!(!glob_match("a/b/c**/*.js", "a/b/c/d/e.js"));
        assert!(glob_match("a/**/b/**/c", "a/b/c/b/c"));
        assert!(glob_match("a/**b**/c", "a/aba/c"));
        assert!(glob_match("a/**b**/c", "a/b/c"));
        assert!(glob_match("a/b/c**/*.js", "a/b/c/d.js"));

        assert!(!glob_match("a/**/*", "a"));
        assert!(!glob_match("a/**/**/*", "a"));
        assert!(!glob_match("a/**/**/**/*", "a"));
        assert!(!glob_match("**/a", "a/"));
        assert!(glob_match("a/**/*", "a/"));
        assert!(glob_match("a/**/**/*", "a/"));
        assert!(glob_match("a/**/**/**/*", "a/"));
        assert!(!glob_match("**/a", "a/b"));
        assert!(!glob_match("a/**/j/**/z/*.md", "a/b/c/j/e/z/c.txt"));
        assert!(!glob_match("a/**/b", "a/bb"));
        assert!(!glob_match("**/a", "a/c"));
        assert!(!glob_match("**/a", "a/b"));
        assert!(!glob_match("**/a", "a/x/y"));
        assert!(!glob_match("**/a", "a/b/c/d"));
        assert!(glob_match("**", "a"));
        assert!(glob_match("**/a", "a"));
        // assert!(glob_match("a/**", "a"));
        assert!(glob_match("**", "a/"));
        assert!(glob_match("**/a/**", "a/"));
        assert!(glob_match("a/**", "a/"));
        assert!(glob_match("a/**/**", "a/"));
        assert!(glob_match("**/a", "a/a"));
        assert!(glob_match("**", "a/b"));
        assert!(glob_match("*/*", "a/b"));
        assert!(glob_match("a/**", "a/b"));
        assert!(glob_match("a/**/*", "a/b"));
        assert!(glob_match("a/**/**/*", "a/b"));
        assert!(glob_match("a/**/**/**/*", "a/b"));
        assert!(glob_match("a/**/b", "a/b"));
        assert!(glob_match("**", "a/b/c"));
        assert!(glob_match("**/*", "a/b/c"));
        assert!(glob_match("**/**", "a/b/c"));
        assert!(glob_match("*/**", "a/b/c"));
        assert!(glob_match("a/**", "a/b/c"));
        assert!(glob_match("a/**/*", "a/b/c"));
        assert!(glob_match("a/**/**/*", "a/b/c"));
        assert!(glob_match("a/**/**/**/*", "a/b/c"));
        assert!(glob_match("**", "a/b/c/d"));
        assert!(glob_match("a/**", "a/b/c/d"));
        assert!(glob_match("a/**/*", "a/b/c/d"));
        assert!(glob_match("a/**/**/*", "a/b/c/d"));
        assert!(glob_match("a/**/**/**/*", "a/b/c/d"));
        assert!(glob_match("a/b/**/c/**/*.*", "a/b/c/d.e"));
        assert!(glob_match("a/**/f/*.md", "a/b/c/d/e/f/g.md"));
        assert!(glob_match("a/**/f/**/k/*.md", "a/b/c/d/e/f/g/h/i/j/k/l.md"));
        assert!(glob_match("a/b/c/*.md", "a/b/c/def.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/ddd.md"));
        assert!(glob_match("a/**/f/*.md", "a/bb.bb/cc/d.d/ee/f/ggg.md"));
        assert!(glob_match("a/**/f/*.md", "a/bb.bb/cc/dd/ee/f/ggg.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb/c/ddd.md"));
        assert!(glob_match("a/*/c/*.md", "a/bbbb/c/ddd.md"));

        assert!(glob_match("foo/bar/**/one/**/*.*", "foo/bar/baz/one/image.png"));
        assert!(glob_match("foo/bar/**/one/**/*.*", "foo/bar/baz/one/two/image.png"));
        assert!(glob_match("foo/bar/**/one/**/*.*", "foo/bar/baz/one/two/three/image.png"));
        assert!(!glob_match("a/b/**/f", "a/b/c/d/"));
        // assert!(glob_match("a/**", "a"));
        assert!(glob_match("**", "a"));
        // assert!(glob_match("a{,/**}", "a"));
        assert!(glob_match("**", "a/"));
        assert!(glob_match("a/**", "a/"));
        assert!(glob_match("**", "a/b/c/d"));
        assert!(glob_match("**", "a/b/c/d/"));
        assert!(glob_match("**/**", "a/b/c/d/"));
        assert!(glob_match("**/b/**", "a/b/c/d/"));
        assert!(glob_match("a/b/**", "a/b/c/d/"));
        assert!(glob_match("a/b/**/", "a/b/c/d/"));
        assert!(glob_match("a/b/**/c/**/", "a/b/c/d/"));
        assert!(glob_match("a/b/**/c/**/d/", "a/b/c/d/"));
        assert!(glob_match("a/b/**/**/*.*", "a/b/c/d/e.f"));
        assert!(glob_match("a/b/**/*.*", "a/b/c/d/e.f"));
        assert!(glob_match("a/b/**/c/**/d/*.*", "a/b/c/d/e.f"));
        assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/e.f"));
        assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/g/e.f"));
        assert!(glob_match("a/b/**/d/**/*.*", "a/b/c/d/g/g/e.f"));
        assert!(glob_match("a/b-*/**/z.js", "a/b-c/z.js"));
        assert!(glob_match("a/b-*/**/z.js", "a/b-c/d/e/z.js"));

        assert!(glob_match("*/*", "a/b"));
        assert!(glob_match("a/b/c/*.md", "a/b/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb.bb/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bb/c/xyz.md"));
        assert!(glob_match("a/*/c/*.md", "a/bbbb/c/xyz.md"));

        assert!(glob_match("**/*", "a/b/c"));
        assert!(glob_match("**/**", "a/b/c"));
        assert!(glob_match("*/**", "a/b/c"));
        assert!(glob_match("a/**/j/**/z/*.md", "a/b/c/d/e/j/n/p/o/z/c.md"));
        assert!(glob_match("a/**/z/*.md", "a/b/c/d/e/z/c.md"));
        assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/b.b/aa/c/xyz.md"));
        assert!(glob_match("a/**/c/*.md", "a/bb.bb/aa/bb/aa/c/xyz.md"));
        assert!(!glob_match("a/**/j/**/z/*.md", "a/b/c/j/e/z/c.txt"));
        assert!(!glob_match("a/b/**/c{d,e}/**/xyz.md", "a/b/c/xyz.md"));
        assert!(!glob_match("a/b/**/c{d,e}/**/xyz.md", "a/b/d/xyz.md"));
        assert!(!glob_match("a/**/", "a/b"));
        // assert!(!glob_match("**/*", "a/b/.js/c.txt"));
        assert!(!glob_match("a/**/", "a/b/c/d"));
        assert!(!glob_match("a/**/", "a/bb"));
        assert!(!glob_match("a/**/", "a/cb"));
        assert!(glob_match("/**", "/a/b"));
        assert!(glob_match("**/*", "a.b"));
        assert!(glob_match("**/*", "a.js"));
        assert!(glob_match("**/*.js", "a.js"));
        // assert!(glob_match("a/**/", "a/"));
        assert!(glob_match("**/*.js", "a/a.js"));
        assert!(glob_match("**/*.js", "a/a/b.js"));
        assert!(glob_match("a/**/b", "a/b"));
        assert!(glob_match("a/**b", "a/b"));
        assert!(glob_match("**/*.md", "a/b.md"));
        assert!(glob_match("**/*", "a/b/c.js"));
        assert!(glob_match("**/*", "a/b/c.txt"));
        assert!(glob_match("a/**/", "a/b/c/d/"));
        assert!(glob_match("**/*", "a/b/c/d/a.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/c/z.js"));
        assert!(glob_match("a/b/**/*.js", "a/b/z.js"));
        assert!(glob_match("**/*", "ab"));
        assert!(glob_match("**/*", "ab/c"));
        assert!(glob_match("**/*", "ab/c/d"));
        assert!(glob_match("**/*", "abc.js"));

        assert!(!glob_match("**/", "a"));
        assert!(!glob_match("**/a/*", "a"));
        assert!(!glob_match("**/a/*/*", "a"));
        assert!(!glob_match("*/a/**", "a"));
        assert!(!glob_match("a/**/*", "a"));
        assert!(!glob_match("a/**/**/*", "a"));
        assert!(!glob_match("**/", "a/b"));
        assert!(!glob_match("**/b/*", "a/b"));
        assert!(!glob_match("**/b/*/*", "a/b"));
        assert!(!glob_match("b/**", "a/b"));
        assert!(!glob_match("**/", "a/b/c"));
        assert!(!glob_match("**/**/b", "a/b/c"));
        assert!(!glob_match("**/b", "a/b/c"));
        assert!(!glob_match("**/b/*/*", "a/b/c"));
        assert!(!glob_match("b/**", "a/b/c"));
        assert!(!glob_match("**/", "a/b/c/d"));
        assert!(!glob_match("**/d/*", "a/b/c/d"));
        assert!(!glob_match("b/**", "a/b/c/d"));
        assert!(glob_match("**", "a"));
        assert!(glob_match("**/**", "a"));
        assert!(glob_match("**/**/*", "a"));
        assert!(glob_match("**/**/a", "a"));
        assert!(glob_match("**/a", "a"));
        // assert!(glob_match("**/a/**", "a"));
        // assert!(glob_match("a/**", "a"));
        assert!(glob_match("**", "a/b"));
        assert!(glob_match("**/**", "a/b"));
        assert!(glob_match("**/**/*", "a/b"));
        assert!(glob_match("**/**/b", "a/b"));
        assert!(glob_match("**/b", "a/b"));
        // assert!(glob_match("**/b/**", "a/b"));
        // assert!(glob_match("*/b/**", "a/b"));
        assert!(glob_match("a/**", "a/b"));
        assert!(glob_match("a/**/*", "a/b"));
        assert!(glob_match("a/**/**/*", "a/b"));
        assert!(glob_match("**", "a/b/c"));
        assert!(glob_match("**/**", "a/b/c"));
        assert!(glob_match("**/**/*", "a/b/c"));
        assert!(glob_match("**/b/*", "a/b/c"));
        assert!(glob_match("**/b/**", "a/b/c"));
        assert!(glob_match("*/b/**", "a/b/c"));
        assert!(glob_match("a/**", "a/b/c"));
        assert!(glob_match("a/**/*", "a/b/c"));
        assert!(glob_match("a/**/**/*", "a/b/c"));
        assert!(glob_match("**", "a/b/c/d"));
        assert!(glob_match("**/**", "a/b/c/d"));
        assert!(glob_match("**/**/*", "a/b/c/d"));
        assert!(glob_match("**/**/d", "a/b/c/d"));
        assert!(glob_match("**/b/**", "a/b/c/d"));
        assert!(glob_match("**/b/*/*", "a/b/c/d"));
        assert!(glob_match("**/d", "a/b/c/d"));
        assert!(glob_match("*/b/**", "a/b/c/d"));
        assert!(glob_match("a/**", "a/b/c/d"));
        assert!(glob_match("a/**/*", "a/b/c/d"));
        assert!(glob_match("a/**/**/*", "a/b/c/d"));

        assert!(glob_match("**/**.txt.js", "/foo/bar.txt.js"));
    }

    #[test]
    fn utf8() {
        assert!(glob_match("フ*/**/*", "フォルダ/aaa.js"));
        assert!(glob_match("フォ*/**/*", "フォルダ/aaa.js"));
        assert!(glob_match("フォル*/**/*", "フォルダ/aaa.js"));
        assert!(glob_match("フ*ル*/**/*", "フォルダ/aaa.js"));
        assert!(glob_match("フォルダ/**/*", "フォルダ/aaa.js"));
    }

    #[test]
    fn negation() {
        assert!(!glob_match("!*", "abc"));
        assert!(!glob_match("!abc", "abc"));
        assert!(!glob_match("*!.md", "bar.md"));
        assert!(!glob_match("foo!.md", "bar.md"));
        assert!(!glob_match("\\!*!*.md", "foo!.md"));
        assert!(!glob_match("\\!*!*.md", "foo!bar.md"));
        assert!(glob_match("*!*.md", "!foo!.md"));
        assert!(glob_match("\\!*!*.md", "!foo!.md"));
        assert!(glob_match("!*foo", "abc"));
        assert!(glob_match("!foo*", "abc"));
        assert!(glob_match("!xyz", "abc"));
        assert!(glob_match("*!*.*", "ba!r.js"));
        assert!(glob_match("*.md", "bar.md"));
        assert!(glob_match("*!*.*", "foo!.md"));
        assert!(glob_match("*!*.md", "foo!.md"));
        assert!(glob_match("*!.md", "foo!.md"));
        assert!(glob_match("*.md", "foo!.md"));
        assert!(glob_match("foo!.md", "foo!.md"));
        assert!(glob_match("*!*.md", "foo!bar.md"));
        assert!(glob_match("*b*.md", "foobar.md"));

        assert!(!glob_match("a!!b", "a"));
        assert!(!glob_match("a!!b", "aa"));
        assert!(!glob_match("a!!b", "a/b"));
        assert!(!glob_match("a!!b", "a!b"));
        assert!(glob_match("a!!b", "a!!b"));
        assert!(!glob_match("a!!b", "a/!!/b"));

        assert!(!glob_match("!a/b", "a/b"));
        assert!(glob_match("!a/b", "a"));
        assert!(glob_match("!a/b", "a.b"));
        assert!(glob_match("!a/b", "a/a"));
        assert!(glob_match("!a/b", "a/c"));
        assert!(glob_match("!a/b", "b/a"));
        assert!(glob_match("!a/b", "b/b"));
        assert!(glob_match("!a/b", "b/c"));

        assert!(!glob_match("!abc", "abc"));
        assert!(glob_match("!!abc", "abc"));
        assert!(!glob_match("!!!abc", "abc"));
        assert!(glob_match("!!!!abc", "abc"));
        assert!(!glob_match("!!!!!abc", "abc"));
        assert!(glob_match("!!!!!!abc", "abc"));
        assert!(!glob_match("!!!!!!!abc", "abc"));
        assert!(glob_match("!!!!!!!!abc", "abc"));

        // assert!(!glob_match("!(*/*)", "a/a"));
        // assert!(!glob_match("!(*/*)", "a/b"));
        // assert!(!glob_match("!(*/*)", "a/c"));
        // assert!(!glob_match("!(*/*)", "b/a"));
        // assert!(!glob_match("!(*/*)", "b/b"));
        // assert!(!glob_match("!(*/*)", "b/c"));
        // assert!(!glob_match("!(*/b)", "a/b"));
        // assert!(!glob_match("!(*/b)", "b/b"));
        // assert!(!glob_match("!(a/b)", "a/b"));
        assert!(!glob_match("!*", "a"));
        assert!(!glob_match("!*", "a.b"));
        assert!(!glob_match("!*/*", "a/a"));
        assert!(!glob_match("!*/*", "a/b"));
        assert!(!glob_match("!*/*", "a/c"));
        assert!(!glob_match("!*/*", "b/a"));
        assert!(!glob_match("!*/*", "b/b"));
        assert!(!glob_match("!*/*", "b/c"));
        assert!(!glob_match("!*/b", "a/b"));
        assert!(!glob_match("!*/b", "b/b"));
        assert!(!glob_match("!*/c", "a/c"));
        assert!(!glob_match("!*/c", "b/c"));
        assert!(!glob_match("!*a*", "bar"));
        assert!(!glob_match("!*a*", "fab"));
        // assert!(!glob_match("!a/(*)", "a/a"));
        // assert!(!glob_match("!a/(*)", "a/b"));
        // assert!(!glob_match("!a/(*)", "a/c"));
        // assert!(!glob_match("!a/(b)", "a/b"));
        assert!(!glob_match("!a/*", "a/a"));
        assert!(!glob_match("!a/*", "a/b"));
        assert!(!glob_match("!a/*", "a/c"));
        assert!(!glob_match("!f*b", "fab"));
        // assert!(glob_match("!(*/*)", "a"));
        // assert!(glob_match("!(*/*)", "a.b"));
        // assert!(glob_match("!(*/b)", "a"));
        // assert!(glob_match("!(*/b)", "a.b"));
        // assert!(glob_match("!(*/b)", "a/a"));
        // assert!(glob_match("!(*/b)", "a/c"));
        // assert!(glob_match("!(*/b)", "b/a"));
        // assert!(glob_match("!(*/b)", "b/c"));
        // assert!(glob_match("!(a/b)", "a"));
        // assert!(glob_match("!(a/b)", "a.b"));
        // assert!(glob_match("!(a/b)", "a/a"));
        // assert!(glob_match("!(a/b)", "a/c"));
        // assert!(glob_match("!(a/b)", "b/a"));
        // assert!(glob_match("!(a/b)", "b/b"));
        // assert!(glob_match("!(a/b)", "b/c"));
        assert!(glob_match("!*", "a/a"));
        assert!(glob_match("!*", "a/b"));
        assert!(glob_match("!*", "a/c"));
        assert!(glob_match("!*", "b/a"));
        assert!(glob_match("!*", "b/b"));
        assert!(glob_match("!*", "b/c"));
        assert!(glob_match("!*/*", "a"));
        assert!(glob_match("!*/*", "a.b"));
        assert!(glob_match("!*/b", "a"));
        assert!(glob_match("!*/b", "a.b"));
        assert!(glob_match("!*/b", "a/a"));
        assert!(glob_match("!*/b", "a/c"));
        assert!(glob_match("!*/b", "b/a"));
        assert!(glob_match("!*/b", "b/c"));
        assert!(glob_match("!*/c", "a"));
        assert!(glob_match("!*/c", "a.b"));
        assert!(glob_match("!*/c", "a/a"));
        assert!(glob_match("!*/c", "a/b"));
        assert!(glob_match("!*/c", "b/a"));
        assert!(glob_match("!*/c", "b/b"));
        assert!(glob_match("!*a*", "foo"));
        // assert!(glob_match("!a/(*)", "a"));
        // assert!(glob_match("!a/(*)", "a.b"));
        // assert!(glob_match("!a/(*)", "b/a"));
        // assert!(glob_match("!a/(*)", "b/b"));
        // assert!(glob_match("!a/(*)", "b/c"));
        // assert!(glob_match("!a/(b)", "a"));
        // assert!(glob_match("!a/(b)", "a.b"));
        // assert!(glob_match("!a/(b)", "a/a"));
        // assert!(glob_match("!a/(b)", "a/c"));
        // assert!(glob_match("!a/(b)", "b/a"));
        // assert!(glob_match("!a/(b)", "b/b"));
        // assert!(glob_match("!a/(b)", "b/c"));
        assert!(glob_match("!a/*", "a"));
        assert!(glob_match("!a/*", "a.b"));
        assert!(glob_match("!a/*", "b/a"));
        assert!(glob_match("!a/*", "b/b"));
        assert!(glob_match("!a/*", "b/c"));
        assert!(glob_match("!f*b", "bar"));
        assert!(glob_match("!f*b", "foo"));

        assert!(!glob_match("!.md", ".md"));
        assert!(glob_match("!**/*.md", "a.js"));
        // assert!(!glob_match("!**/*.md", "b.md"));
        assert!(glob_match("!**/*.md", "c.txt"));
        assert!(glob_match("!*.md", "a.js"));
        assert!(!glob_match("!*.md", "b.md"));
        assert!(glob_match("!*.md", "c.txt"));
        assert!(!glob_match("!*.md", "abc.md"));
        assert!(glob_match("!*.md", "abc.txt"));
        assert!(!glob_match("!*.md", "foo.md"));
        assert!(glob_match("!.md", "foo.md"));

        assert!(glob_match("!*.md", "a.js"));
        assert!(glob_match("!*.md", "b.txt"));
        assert!(!glob_match("!*.md", "c.md"));
        assert!(!glob_match("!a/*/a.js", "a/a/a.js"));
        assert!(!glob_match("!a/*/a.js", "a/b/a.js"));
        assert!(!glob_match("!a/*/a.js", "a/c/a.js"));
        assert!(!glob_match("!a/*/*/a.js", "a/a/a/a.js"));
        assert!(glob_match("!a/*/*/a.js", "b/a/b/a.js"));
        assert!(glob_match("!a/*/*/a.js", "c/a/c/a.js"));
        assert!(!glob_match("!a/a*.txt", "a/a.txt"));
        assert!(glob_match("!a/a*.txt", "a/b.txt"));
        assert!(glob_match("!a/a*.txt", "a/c.txt"));
        assert!(!glob_match("!a.a*.txt", "a.a.txt"));
        assert!(glob_match("!a.a*.txt", "a.b.txt"));
        assert!(glob_match("!a.a*.txt", "a.c.txt"));
        assert!(!glob_match("!a/*.txt", "a/a.txt"));
        assert!(!glob_match("!a/*.txt", "a/b.txt"));
        assert!(!glob_match("!a/*.txt", "a/c.txt"));

        assert!(glob_match("!*.md", "a.js"));
        assert!(glob_match("!*.md", "b.txt"));
        assert!(!glob_match("!*.md", "c.md"));
        // assert!(!glob_match("!**/a.js", "a/a/a.js"));
        // assert!(!glob_match("!**/a.js", "a/b/a.js"));
        // assert!(!glob_match("!**/a.js", "a/c/a.js"));
        assert!(glob_match("!**/a.js", "a/a/b.js"));
        assert!(!glob_match("!a/**/a.js", "a/a/a/a.js"));
        assert!(glob_match("!a/**/a.js", "b/a/b/a.js"));
        assert!(glob_match("!a/**/a.js", "c/a/c/a.js"));
        assert!(glob_match("!**/*.md", "a/b.js"));
        assert!(glob_match("!**/*.md", "a.js"));
        assert!(!glob_match("!**/*.md", "a/b.md"));
        // assert!(!glob_match("!**/*.md", "a.md"));
        assert!(!glob_match("**/*.md", "a/b.js"));
        assert!(!glob_match("**/*.md", "a.js"));
        assert!(glob_match("**/*.md", "a/b.md"));
        assert!(glob_match("**/*.md", "a.md"));
        assert!(glob_match("!**/*.md", "a/b.js"));
        assert!(glob_match("!**/*.md", "a.js"));
        assert!(!glob_match("!**/*.md", "a/b.md"));
        // assert!(!glob_match("!**/*.md", "a.md"));
        assert!(glob_match("!*.md", "a/b.js"));
        assert!(glob_match("!*.md", "a.js"));
        assert!(glob_match("!*.md", "a/b.md"));
        assert!(!glob_match("!*.md", "a.md"));
        assert!(glob_match("!**/*.md", "a.js"));
        // assert!(!glob_match("!**/*.md", "b.md"));
        assert!(glob_match("!**/*.md", "c.txt"));
    }

    #[test]
    fn question_mark() {
        assert!(glob_match("?", "a"));
        assert!(!glob_match("?", "aa"));
        assert!(!glob_match("?", "ab"));
        assert!(!glob_match("?", "aaa"));
        assert!(!glob_match("?", "abcdefg"));

        assert!(!glob_match("??", "a"));
        assert!(glob_match("??", "aa"));
        assert!(glob_match("??", "ab"));
        assert!(!glob_match("??", "aaa"));
        assert!(!glob_match("??", "abcdefg"));

        assert!(!glob_match("???", "a"));
        assert!(!glob_match("???", "aa"));
        assert!(!glob_match("???", "ab"));
        assert!(glob_match("???", "aaa"));
        assert!(!glob_match("???", "abcdefg"));

        assert!(!glob_match("a?c", "aaa"));
        assert!(glob_match("a?c", "aac"));
        assert!(glob_match("a?c", "abc"));
        assert!(!glob_match("ab?", "a"));
        assert!(!glob_match("ab?", "aa"));
        assert!(!glob_match("ab?", "ab"));
        assert!(!glob_match("ab?", "ac"));
        assert!(!glob_match("ab?", "abcd"));
        assert!(!glob_match("ab?", "abbb"));
        assert!(glob_match("a?b", "acb"));

        assert!(!glob_match("a/?/c/?/e.md", "a/bb/c/dd/e.md"));
        assert!(glob_match("a/??/c/??/e.md", "a/bb/c/dd/e.md"));
        assert!(!glob_match("a/??/c.md", "a/bbb/c.md"));
        assert!(glob_match("a/?/c.md", "a/b/c.md"));
        assert!(glob_match("a/?/c/?/e.md", "a/b/c/d/e.md"));
        assert!(!glob_match("a/?/c/???/e.md", "a/b/c/d/e.md"));
        assert!(glob_match("a/?/c/???/e.md", "a/b/c/zzz/e.md"));
        assert!(!glob_match("a/?/c.md", "a/bb/c.md"));
        assert!(glob_match("a/??/c.md", "a/bb/c.md"));
        assert!(glob_match("a/???/c.md", "a/bbb/c.md"));
        assert!(glob_match("a/????/c.md", "a/bbbb/c.md"));
    }

    #[test]
    fn braces() {
        assert!(glob_match("{a,b,c}", "a"));
        assert!(glob_match("{a,b,c}", "b"));
        assert!(glob_match("{a,b,c}", "c"));
        assert!(!glob_match("{a,b,c}", "aa"));
        assert!(!glob_match("{a,b,c}", "bb"));
        assert!(!glob_match("{a,b,c}", "cc"));

        assert!(glob_match("a/{a,b}", "a/a"));
        assert!(glob_match("a/{a,b}", "a/b"));
        assert!(!glob_match("a/{a,b}", "a/c"));
        assert!(!glob_match("a/{a,b}", "b/b"));
        assert!(!glob_match("a/{a,b,c}", "b/b"));
        assert!(glob_match("a/{a,b,c}", "a/c"));
        assert!(glob_match("a{b,bc}.txt", "abc.txt"));

        assert!(glob_match("foo[{a,b}]baz", "foo{baz"));

        assert!(!glob_match("a{,b}.txt", "abc.txt"));
        assert!(!glob_match("a{a,b,}.txt", "abc.txt"));
        assert!(!glob_match("a{b,}.txt", "abc.txt"));
        assert!(glob_match("a{,b}.txt", "a.txt"));
        assert!(glob_match("a{b,}.txt", "a.txt"));
        assert!(glob_match("a{a,b,}.txt", "aa.txt"));
        assert!(glob_match("a{,b}.txt", "ab.txt"));
        assert!(glob_match("a{b,}.txt", "ab.txt"));

        // assert!(glob_match("{a/,}a/**", "a"));
        assert!(glob_match("a{a,b/}*.txt", "aa.txt"));
        assert!(glob_match("a{a,b/}*.txt", "ab/.txt"));
        assert!(glob_match("a{a,b/}*.txt", "ab/a.txt"));
        // assert!(glob_match("{a/,}a/**", "a/"));
        assert!(glob_match("{a/,}a/**", "a/a/"));
        // assert!(glob_match("{a/,}a/**", "a/a"));
        assert!(glob_match("{a/,}a/**", "a/a/a"));
        assert!(glob_match("{a/,}a/**", "a/a/"));
        assert!(glob_match("{a/,}a/**", "a/a/a/"));
        assert!(glob_match("{a/,}b/**", "a/b/a/"));
        assert!(glob_match("{a/,}b/**", "b/a/"));
        assert!(glob_match("a{,/}*.txt", "a.txt"));
        assert!(glob_match("a{,/}*.txt", "ab.txt"));
        assert!(glob_match("a{,/}*.txt", "a/b.txt"));
        assert!(glob_match("a{,/}*.txt", "a/ab.txt"));

        assert!(glob_match("a{,.*{foo,db},\\(bar\\)}.txt", "a.txt"));
        assert!(!glob_match("a{,.*{foo,db},\\(bar\\)}.txt", "adb.txt"));
        assert!(glob_match("a{,.*{foo,db},\\(bar\\)}.txt", "a.db.txt"));

        assert!(glob_match("a{,*.{foo,db},\\(bar\\)}.txt", "a.txt"));
        assert!(!glob_match("a{,*.{foo,db},\\(bar\\)}.txt", "adb.txt"));
        assert!(glob_match("a{,*.{foo,db},\\(bar\\)}.txt", "a.db.txt"));

        // assert!(glob_match("a{,.*{foo,db},\\(bar\\)}", "a"));
        assert!(!glob_match("a{,.*{foo,db},\\(bar\\)}", "adb"));
        assert!(glob_match("a{,.*{foo,db},\\(bar\\)}", "a.db"));

        // assert!(glob_match("a{,*.{foo,db},\\(bar\\)}", "a"));
        assert!(!glob_match("a{,*.{foo,db},\\(bar\\)}", "adb"));
        assert!(glob_match("a{,*.{foo,db},\\(bar\\)}", "a.db"));

        assert!(!glob_match("{,.*{foo,db},\\(bar\\)}", "a"));
        assert!(!glob_match("{,.*{foo,db},\\(bar\\)}", "adb"));
        assert!(!glob_match("{,.*{foo,db},\\(bar\\)}", "a.db"));
        assert!(glob_match("{,.*{foo,db},\\(bar\\)}", ".db"));

        assert!(!glob_match("{,*.{foo,db},\\(bar\\)}", "a"));
        assert!(glob_match("{*,*.{foo,db},\\(bar\\)}", "a"));
        assert!(!glob_match("{,*.{foo,db},\\(bar\\)}", "adb"));
        assert!(glob_match("{,*.{foo,db},\\(bar\\)}", "a.db"));

        assert!(!glob_match("a/b/**/c{d,e}/**/xyz.md", "a/b/c/xyz.md"));
        assert!(!glob_match("a/b/**/c{d,e}/**/xyz.md", "a/b/d/xyz.md"));
        assert!(glob_match("a/b/**/c{d,e}/**/xyz.md", "a/b/cd/xyz.md"));
        assert!(glob_match("a/b/**/{c,d,e}/**/xyz.md", "a/b/c/xyz.md"));
        assert!(glob_match("a/b/**/{c,d,e}/**/xyz.md", "a/b/d/xyz.md"));
        assert!(glob_match("a/b/**/{c,d,e}/**/xyz.md", "a/b/e/xyz.md"));

        assert!(glob_match("*{a,b}*", "xax"));
        assert!(glob_match("*{a,b}*", "xxax"));
        assert!(glob_match("*{a,b}*", "xbx"));

        assert!(glob_match("*{*a,b}", "xba"));
        assert!(glob_match("*{*a,b}", "xb"));

        assert!(!glob_match("*??", "a"));
        assert!(!glob_match("*???", "aa"));
        assert!(glob_match("*???", "aaa"));
        assert!(!glob_match("*****??", "a"));
        assert!(!glob_match("*****???", "aa"));
        assert!(glob_match("*****???", "aaa"));

        assert!(!glob_match("a*?c", "aaa"));
        assert!(glob_match("a*?c", "aac"));
        assert!(glob_match("a*?c", "abc"));

        assert!(glob_match("a**?c", "abc"));
        assert!(!glob_match("a**?c", "abb"));
        assert!(glob_match("a**?c", "acc"));
        assert!(glob_match("a*****?c", "abc"));

        assert!(glob_match("*****?", "a"));
        assert!(glob_match("*****?", "aa"));
        assert!(glob_match("*****?", "abc"));
        assert!(glob_match("*****?", "zzz"));
        assert!(glob_match("*****?", "bbb"));
        assert!(glob_match("*****?", "aaaa"));

        assert!(!glob_match("*****??", "a"));
        assert!(glob_match("*****??", "aa"));
        assert!(glob_match("*****??", "abc"));
        assert!(glob_match("*****??", "zzz"));
        assert!(glob_match("*****??", "bbb"));
        assert!(glob_match("*****??", "aaaa"));

        assert!(!glob_match("?*****??", "a"));
        assert!(!glob_match("?*****??", "aa"));
        assert!(glob_match("?*****??", "abc"));
        assert!(glob_match("?*****??", "zzz"));
        assert!(glob_match("?*****??", "bbb"));
        assert!(glob_match("?*****??", "aaaa"));

        assert!(glob_match("?*****?c", "abc"));
        assert!(!glob_match("?*****?c", "abb"));
        assert!(!glob_match("?*****?c", "zzz"));

        assert!(glob_match("?***?****c", "abc"));
        assert!(!glob_match("?***?****c", "bbb"));
        assert!(!glob_match("?***?****c", "zzz"));

        assert!(glob_match("?***?****?", "abc"));
        assert!(glob_match("?***?****?", "bbb"));
        assert!(glob_match("?***?****?", "zzz"));

        assert!(glob_match("?***?****", "abc"));
        assert!(glob_match("*******c", "abc"));
        assert!(glob_match("*******?", "abc"));
        assert!(glob_match("a*cd**?**??k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??k***", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??***k", "abcdecdhjk"));
        assert!(glob_match("a**?**cd**?**??***k**", "abcdecdhjk"));
        assert!(glob_match("a****c**?**??*****", "abcdecdhjk"));

        assert!(!glob_match("a/?/c/?/*/e.md", "a/b/c/d/e.md"));
        assert!(glob_match("a/?/c/?/*/e.md", "a/b/c/d/e/e.md"));
        assert!(glob_match("a/?/c/?/*/e.md", "a/b/c/d/efghijk/e.md"));
        assert!(glob_match("a/?/**/e.md", "a/b/c/d/efghijk/e.md"));
        assert!(!glob_match("a/?/e.md", "a/bb/e.md"));
        assert!(glob_match("a/??/e.md", "a/bb/e.md"));
        assert!(!glob_match("a/?/**/e.md", "a/bb/e.md"));
        assert!(glob_match("a/?/**/e.md", "a/b/ccc/e.md"));
        assert!(glob_match("a/*/?/**/e.md", "a/b/c/d/efghijk/e.md"));
        assert!(glob_match("a/*/?/**/e.md", "a/b/c/d/efgh.ijk/e.md"));
        assert!(glob_match("a/*/?/**/e.md", "a/b.bb/c/d/efgh.ijk/e.md"));
        assert!(glob_match("a/*/?/**/e.md", "a/bbb/c/d/efgh.ijk/e.md"));

        assert!(glob_match("a/*/ab??.md", "a/bbb/abcd.md"));
        assert!(glob_match("a/bbb/ab??.md", "a/bbb/abcd.md"));
        assert!(glob_match("a/bbb/ab???md", "a/bbb/abcd.md"));

        assert!(glob_match("{a,b}/c/{d,e}/**/*.ts", "a/c/d/one/two/three.test.ts"));
        assert!(glob_match("{a,{d,e}b}/c", "a/c"));
        assert!(glob_match("{**/a,**/b}", "b"));

        let patterns = ["{src,extensions}/**/test/**/{fixtures,browser,common}/**/*.{ts,js}",
      "{extensions,src}/**/{media,images,icons}/**/*.{svg,png,gif,jpg}",
      "{.github,build,test}/**/{workflows,azure-pipelines,integration,smoke}/**/*.{yml,yaml,json}",
      "src/vs/{base,editor,platform,workbench}/test/{browser,common,node}/**/[a-z]*[tT]est.ts",
      "src/vs/workbench/{contrib,services}/**/*{Editor,Workspace,Terminal}*.ts",
      "{extensions,src}/**/{markdown,json,javascript,typescript}/**/*.{ts,json}",
      "**/{electron-sandbox,electron-main,browser,node}/**/{*[sS]ervice*,*[cC]ontroller*}.ts",
      "{src,extensions}/**/{common,browser,electron-sandbox}/**/*{[cC]ontribution,[sS]ervice}.ts",
      "src/vs/{base,platform,workbench}/**/{test,browser}/**/*{[mM]odel,[cC]ontroller}*.ts",
      "extensions/**/{browser,common,node}/{**/*[sS]ervice*,**/*[pP]rovider*}.ts"];

        let input = std::fs::read_to_string("tests/fixtures/input.txt").unwrap();

        for (i, pattern) in patterns.iter().enumerate() {
            let mut matched: Vec<&str> = vec![];
            for line in input.lines() {
                if glob_match(pattern, line) {
                    matched.push(line);
                }
            }

            let expected_matches =
                std::fs::read_to_string(format!("tests/fixtures/matched-pattern-{}.txt", i + 1))
                    .unwrap();
            let expected = expected_matches.lines().collect::<Vec<&str>>();
            assert_eq!(matched, expected);
        }
    }

    #[test]
    fn not_paired_braces() {
        assert!(!glob_match("{a,}}", "a"));
        assert!(glob_match("{a,}}", "a}"));
    }

    #[test]
    fn fuzz_tests() {
        // https://github.com/devongovett/glob-match/issues/1
        let s = "{*{??*{??**,Uz*zz}w**{*{**a,z***b*[!}w??*azzzzzzzz*!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!z[za,z&zz}w**z*z*}";
        assert!(!glob_match(s, s));
        let s = "**** *{*{??*{??***\u{5} *{*{??*{??***\u{5},\0U\0}]*****\u{1},\0***\0,\0\0}w****,\0U\0}]*****\u{1},\0***\0,\0\0}w*****\u{1}***{}*.*\0\0*\0";
        assert!(!glob_match(s, s));
    }
}
