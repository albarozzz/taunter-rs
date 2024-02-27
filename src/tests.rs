#[cfg(test)]
mod testing {
    use crate::helper::*;
    use lazy_static::lazy_static;

    static LINE: &str =
        r#"03/11/2023 - 01:55:04: ./albarozzz killed Grim Bloody Fable with force_a_nature."#;
    lazy_static! {
        static ref USERNAMES: Vec<String> = vec!["./albarozzz".to_string()];
        static ref USERNAME_VICTIM: Vec<String> = vec!["Grim Bloody Fable".to_string()];
    }

    #[test]
    fn check_test() {
        let (username, victim) = get_usernames(LINE);
        let result = check(&USERNAMES, &USERNAME_VICTIM, &username, &victim);

        assert!(result);
        assert_eq!(&username, "./albarozzz");
        assert_eq!(&victim, "Grim Bloody Fable");
    }

    #[test]
    fn check_test_should_return_false() {
        let username_victim_wrong = vec!["Mentlegen".to_string()];

        let (username, victim) = get_usernames(LINE);
        let result = check(&USERNAMES, &username_victim_wrong, &username, &victim);

        assert!(!result);
        assert_eq!(&username, "./albarozzz");
        assert_ne!(&victim, "Mentlegen");
    }

    #[test]
    fn get_usernames_should_return_empty_strings() {
        let (username, victim) = get_usernames("annoyingbug: madagascar killed albaro");
        let result = check(&USERNAMES, &USERNAME_VICTIM, &username, &victim);

        assert!(!result);
        assert_eq!(&username, "");
        assert_eq!(&victim, "");
        assert_ne!(&username, "madagascar");
        assert_ne!(&victim, "albaro");
    }
}
