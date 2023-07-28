#[cfg(test)]
mod from_trait {
    use crate::git::{Flag, Hook};
    #[test]
    fn hook_to_string() {
        let hook = Hook::ApplypatchMsg;
        let res = String::from(&hook);
        let string = "applypatch-msg".to_owned();
        assert_eq!(string, res);
    }
    #[test]
    fn string_to_hook() {
        let string = "applypatch-msg".to_owned();
        let res = Hook::from(&string);
        let hook = Hook::ApplypatchMsg;
        assert_eq!(hook, res);
    }
    #[test]
    #[should_panic]
    fn bag_string_to_flag() {
        let string = "unknown-flag".to_owned();
        let res = Flag::from(&string);
    }
}
