enum State {
    Default,
    StartComment,
    InLineComment,
    InMultilineComment,
    EndingMultilineComment,
}

// I think this may produce different results for something like
// "/**/*/". I think the regex version will turn it into "      ",
// but the manual version will turn it into "    */".
pub fn uncomment(text: &str) -> String {
    let mut state = State::Default;
    let mut s = String::with_capacity(text.len());

    for c in text.chars() {
        match (state, c) {
            (State::Default, '/') => state = State::StartComment,
            // As a hack to maximize our ability to process files like
            // mozilla-central's PContent.ipdl, we currently treat preprocessor
            // directives like they are comments.  This is not sound from a
            // correctness perspective (but also you should not be feeding this
            // parser preprocessing directives; you should feed it the output of
            // a preprocessor!).  Bug 1859884 tracks improving this behavior, at
            // which time this handling of "#" can potentially be removed and go
            // back to causing parse errors.
            (State::Default, '#') => {
                s.push(' ');
                state = State::InLineComment;
            }
            (State::Default, _) => {
                s.push(c);
                state = State::Default;
            }
            (State::StartComment, '/') => {
                s.push(' ');
                s.push(' ');
                state = State::InLineComment;
            }
            (State::StartComment, '*') => {
                s.push(' ');
                s.push(' ');
                state = State::InMultilineComment;
            }
            (State::StartComment, _) => {
                s.push('/');
                s.push(c);
                state = State::Default;
            }
            (State::InLineComment, '\n') => {
                s.push('\n');
                state = State::Default;
            }
            (State::InLineComment, _) => {
                s.push(' ');
                state = State::InLineComment;
            }
            (State::InMultilineComment, _) => {
                s.push(if c == '\n' { '\n' } else { ' ' });
                if c == '*' {
                    state = State::EndingMultilineComment;
                } else {
                    state = State::InMultilineComment;
                }
            }
            (State::EndingMultilineComment, _) => {
                s.push(if c == '\n' { '\n' } else { ' ' });
                if c == '/' {
                    state = State::Default;
                } else if c == '*' {
                    state = State::EndingMultilineComment;
                } else {
                    state = State::InMultilineComment;
                }
            }
        }
    }

    s
}

#[test]
fn basic_tests() {
    assert_eq!(uncomment("123"), "123");
    assert_eq!(uncomment("//123"), "     ");
    assert_eq!(uncomment("//123\n"), "     \n");
    assert_eq!(uncomment("//123\n45"), "     \n45");
    assert_eq!(uncomment("//123\n45"), "     \n45");
    assert_eq!(uncomment("//123\n//45\n6"), "     \n    \n6");

    // Preprocessor directives get replaced with spaces (like comments).
    assert_eq!(
        uncomment("#ifdef foo\nblah\n#endif\ngrah"),
        "          \nblah\n      \ngrah"
    );
    // Preproc directives inside `//` comments are treated as comments.
    assert_eq!(
        uncomment("//#ifdef foo\nblah\n// #endif\ngrah"),
        "            \nblah\n         \ngrah"
    );
    // Preproc directives inside `/* */` comments are treated as comments.
    assert_eq!(
        uncomment("/* #ifdef foo*/\nblah\n/*#endif*/\ngrah"),
        "               \nblah\n          \ngrah"
    );

    assert_eq!(uncomment("0/*123*/0"), "0       0");
    assert_eq!(uncomment("0/*12\n3*/0"), "0    \n   0");

    // Newline right before fake end of multiline comment.
    assert_eq!(uncomment("/**\n*/0"), "   \n  0");

    // After we get a * in the middle of a multiline comment, that is
    // not followed by a /, we need to reset to the normal
    // in-multiline-comment state. Funnily enough, this is necessary
    // to correctly parse the multiline comment version of the MPL2.
    assert_eq!(uncomment("/**x/y*/0"), "        0");

    assert_eq!(uncomment("/* ... **/123"), "          123");
}
