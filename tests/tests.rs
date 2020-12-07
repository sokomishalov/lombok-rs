// fixme
#![feature(derive_eq)]
#![feature(structural_match)]

#[macro_use]
extern crate lombok_derive;
extern crate lombok_trait;

use lombok_trait::NoArgsContructor;

#[derive(
    Getter, Setter, AllArgsConstructor, NoArgsConstructor, Builder, EqualsAndHashcode, ToString,
)]
pub struct TestNamedStructure<'a> {
    age: u8,
    nick: &'a str,
    languages: Vec<String>,
    hobby: Box<String>,
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::TestNamedStructure;

    #[test]
    fn test_getters() {
        let data = TestNamedStructure {
            age: 25,
            nick: "sokomishalov",
            languages: vec!["rust".to_string(), "kotlin".to_string()],
            hobby: Box::new("soccer".to_string()),
        };

        assert_eq!(&data.age, data.get_age());
        assert_eq!(&data.nick, &data.get_nick());
        assert_eq!(&data.languages, data.get_languages());
        assert_eq!(&data.hobby, data.get_hobby());
    }
}
