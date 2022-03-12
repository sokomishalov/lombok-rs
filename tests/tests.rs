use lombok::{
    AllArgsConstructor, Builder, EqualsAndHashCode, Getter, GetterMut, NoArgsConstructor, Setter,
    ToString,
};

#[derive(
    Getter,
    GetterMut,
    Setter,
    NoArgsConstructor,
    AllArgsConstructor,
    EqualsAndHashCode,
    Builder,
    ToString,
    Clone,
)]
pub struct TestNamedStructure<'a> {
    age: u8,
    nick: &'a str,
    languages: Vec<String>,
    hobby: Box<&'a str>,
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::TestNamedStructure;

    #[test]
    fn test_getter() {
        let default = TestNamedStructure::default();

        assert_eq!(&default.age, default.get_age());
        assert_eq!(default.nick, default.get_nick());
        assert_eq!(&default.languages, default.get_languages());
        assert_eq!(&default.hobby, default.get_hobby());
    }

    #[test]
    fn test_getter_mut() {
        let mut default = TestNamedStructure::default();
        *default.get_age_mut() = 10;
        *default.get_nick_mut() = "another";
        *default.get_languages_mut() = vec!["python".to_string()];
        *default.get_hobby_mut() = Box::new("tennis");

        let copy = TestNamedStructure::default();

        assert_ne!(copy.age, default.age);
        assert_ne!(copy.nick, default.nick);
        assert_ne!(copy.languages, default.languages);
        assert_ne!(copy.hobby, default.hobby);
    }

    #[test]
    fn test_setter() {
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

    #[test]
    fn test_equals_and_hashcode() {
        let default = TestNamedStructure::default();
        let mut clone_default = default.clone();

        assert!(&default.equals(&clone_default));
        assert_eq!(&default.hash_code(), &clone_default.hash_code());

        clone_default.age = 10;

        assert!(!&default.equals(&clone_default));
        assert_ne!(&default.hash_code(), &clone_default.hash_code());
    }

    #[test]
    fn test_to_string() {
        let default = TestNamedStructure::default();
        let string = default.to_string();
        assert_eq!(
            String::from("TestNamedStructure { age: 0, nick: \"\", languages: [], hobby: \"\" }"),
            string
        )
    }

    #[test]
    fn test_builder() {
        let default = TestNamedStructure::default();
        let clone_default = default.clone();

        let data = TestNamedStructure::builder()
            .age(clone_default.age)
            .nick(clone_default.nick)
            .languages(clone_default.languages)
            .hobby(clone_default.hobby)
            .build();

        assert_eq!(&default, &data);
    }
}
