#[cfg(test)]
mod testing {
    use crate::helper::*;

    #[test]
    fn check_test() {
        let line =
            r#"03/11/2023 - 01:55:04: ./albarozzz killed Grim Bloody Fable with force_a_nature."#;
        let usernames = vec!["./albarozzz".to_string()];
        let username_victim = vec!["Grim Bloody Fable".to_string()];

        let (result, username, victim) = check(&usernames, &username_victim, line);

        assert!(result);
        assert_eq!(&username, "./albarozzz");
        assert_eq!(&victim, "Grim Bloody Fable");
    }

    #[test]
    fn check_test_should_return_false() {
        let line = r#"03/11/2023 - 01:55:04: ./albarozzz killed Pedorrito with force_a_nature."#;
        let usernames = vec!["./albarozzz".to_string()];
        let username_victim = vec!["Mentlegen".to_string()];

        let (result, username, victim) = check(&usernames, &username_victim, line);

        assert!(!result);
        assert_eq!(&username, "./albarozzz");
        assert_ne!(&victim, "Mentlegen");
    }
}
