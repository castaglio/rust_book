// use minigrep::search_insensitive;

#[test]
fn it_case_insensitive_three() {
    let content = "
Hola radiola, safe
hola RADIOLA, manteca,
\
colora";

    assert_eq!(vec!["Hola radiola, safe","hola RADIOLA, manteca,"], minigrep::search_insensitive("hola", content));
}
