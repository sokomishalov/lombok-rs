// fixme
#![feature(derive_eq)]
#![feature(structural_match)]

#[macro_use]
extern crate lombok_derive;
extern crate lombok_trait;

#[derive(Getter, Setter, NoArgsConstructor, Builder, EqualsAndHashcode, ToString, Clone)]
pub struct TestNamedStructure<'a> {
    age: u8,
    nick: &'a str,
    languages: Vec<String>,
    hobby: Box<String>,
}

impl Default for TestNamedStructure<'_> {
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
    use lombok_trait::NoArgsContructor;

    use super::TestNamedStructure;

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
        let mut data = TestNamedStructure::new();

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
}
