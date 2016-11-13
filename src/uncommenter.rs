use regex::Regex;
use regex::Captures;

pub fn uncomment(text: &str) -> String {
    let re = Regex::new(r"((?://[^\n]*)|(?:/\*(\n|.)*?\*/))").unwrap();
    re.replace_all(text, |caps: &Captures| {
        let orig_s = caps.at(1).unwrap();
        let mut s = String::with_capacity(orig_s.len());
        for c in orig_s.chars() {
            s.push(if c == '\n' { '\n' } else { ' ' });
        };
        s
    })
}

#[test]
fn basic_tests() {
    assert_eq!(uncomment("123"), "123");
    assert_eq!(uncomment("//123"), "     ");
    assert_eq!(uncomment("//123\n"), "     \n");
    assert_eq!(uncomment("//123\n45"), "     \n45");
    assert_eq!(uncomment("//123\n45"), "     \n45");
    assert_eq!(uncomment("//123\n//45\n6"), "     \n    \n6");

    assert_eq!(uncomment("0/*123*/0"), "0       0");
    assert_eq!(uncomment("0/*12\n3*/0"), "0    \n   0");
}
