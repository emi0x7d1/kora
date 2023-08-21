use kora_lexer::Lexer;

#[test]
fn test_lexer() {
    insta::glob!("inputs/*.kora", |path| {
        let input = std::fs::read_to_string(path).unwrap();

        let tokens = Lexer::new(&input).collect::<Vec<_>>();

        insta::with_settings!({
            description => &input,
            omit_expression => true,
        }, {
            insta::assert_debug_snapshot!(tokens);
        });
    })
}
