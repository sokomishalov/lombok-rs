// fixme
#![feature(derive_eq)]
#![feature(structural_match)]

use lombok::{AllArgsContructor, Builder, EqualsAndHashcode, Getter, NoArgsConstructor, Setter};

#[derive(
    Getter, Setter, NoArgsConstructor, AllArgsContructor, Builder, EqualsAndHashcode, Clone,
)]
pub struct TestNamedStructure<'a> {
    age: u8,
    nick: &'a str,
    languages: Vec<String>,
    hobby: Box<String>,
}

impl Default for TestNamedStructure<'static> {
    fn default() -> Self {
        TestNamedStructure {
            age: 25,
            nick: "sokomishalov",
            languages: vec!["rust".to_string(), "kotlin".to_string()],
            hobby: Box::new("soccer".to_string()),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::TestNamedStructure;

    #[test]
    fn test_getters() {
        let default = TestNamedStructure::default();

        assert_eq!(&default.age, default.get_age());
        assert_eq!(default.nick, default.get_nick());
        assert_eq!(&default.languages, default.get_languages());
        assert_eq!(&default.hobby, default.get_hobby());
    }

    #[test]
    fn test_setters() {
        let mut data = TestNamedStructure {
            age: Default::default(),
            nick: Default::default(),
            languages: Default::default(),
            hobby: Default::default(),
        };

        let default = TestNamedStructure::default();
        let clone_default = default.clone();

        data.set_age(clone_default.age);
        data.set_nick(clone_default.nick);
        data.set_languages(clone_default.languages);
        data.set_hobby(clone_default.hobby);

        assert_eq!(default.age, data.age);
        assert_eq!(default.nick, data.nick);
        assert_eq!(default.languages, data.languages);
        assert_eq!(default.hobby, data.hobby);
    }

    #[test]
    fn test_no_args_constructor() {
        let mut data = TestNamedStructure::new_default();

        let default = TestNamedStructure::default();
        let clone_default = default.clone();

        data.set_age(clone_default.age);
        data.set_nick(clone_default.nick);
        data.set_languages(clone_default.languages);
        data.set_hobby(clone_default.hobby);

        assert_eq!(default.age, data.age);
        assert_eq!(default.nick, data.nick);
        assert_eq!(default.languages, data.languages);
        assert_eq!(default.hobby, data.hobby);
    }

    #[test]
    fn test_all_args_constructor() {
        let default = TestNamedStructure::default();
        let clone_default = default.clone();

        let data = TestNamedStructure::new(
            clone_default.age,
            clone_default.nick,
            clone_default.languages,
            clone_default.hobby,
        );

        assert_eq!(default.age, data.age);
        assert_eq!(default.nick, data.nick);
        assert_eq!(default.languages, data.languages);
        assert_eq!(default.hobby, data.hobby);
    }
}
