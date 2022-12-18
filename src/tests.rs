#[cfg(test)]
mod testing {
    use crate::helper::*;

    #[test]
    fn check_test() {
        let line = r#"L 12/12/2022 - 20:41:37: "./albarozzz<2><[U:1:383329786]><Blue>" killed "Grim Bloody Fable<3><BOT><Red>" with "force_a_nature" (attacker_position "-5301 8028 -191") (victim_position "-5053 8100 -108")"#;
        let usernames = vec!["./albarozzz".to_string()];
        let username_victim = vec!["Grim Bloody Fable".to_string()];

        let result = check(&usernames, &username_victim, line);

        assert!(result);
    }

    #[test]
    fn check_test_should_return_false() {
        let line = r#"L 12/12/2022 - 20:41:37: doesn't matter"#;
        let usernames = vec!["./albarozzz".to_string()];
        let username_victim = vec!["Grim Bloody Fable".to_string()];

        let result = check(&usernames, &username_victim, line);

        assert!(!result);
    }

    #[test]
    fn check_test_complex() {
        let line = r#"L 12/12/2022 - 20:41:37: "./albarozzz killed pedorrito with scattergun<2><[U:1:383329786]><Blue>" killed "Grim Bloody Fable<3><BOT><Red>" with "force_a_nature" (attacker_position "-5301 8028 -191") (victim_position "-5053 8100 -108")"#;
        let usernames = vec!["./albarozzz killed pedorrito with scattergun".to_string()];
        let username_victim = vec!["Grim Bloody Fable".to_string()];

        let result = check(&usernames, &username_victim, line);

        assert!(result);
    }

    #[test]
    fn check_test_complex_should_return_false() {
        let line = r#"L 12/12/2022 - 20:41:37: "albaro" killed "pedorrito", "?", "??", "???""#;
        let usernames = vec!["./albarozzz".to_string()];
        let username_victim = vec!["Grim Bloody Fable".to_string()];

        let result = check(&usernames, &username_victim, line);

        assert!(!result);
    }
}
