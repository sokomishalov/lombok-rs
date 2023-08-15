use lombok::{
    AllArgsConstructor, Builder, Data, EqualsAndHashCode, Getter, GetterMut, NoArgsConstructor,
    Setter, ToString, Value,
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
pub struct TestFullFeaturedStructure<'a> {
    age: u8,
    nick: &'a str,
    languages: Vec<String>,
    hobby: Box<&'a str>,
}

#[derive(Data)]
struct TestDataStructure<'a> {
    age: u8,
    nick: &'a str,
}

#[derive(Value)]
struct TestValueStructure<'a> {
    age: u8,
    nick: &'a str,
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{TestDataStructure, TestFullFeaturedStructure, TestValueStructure};

    #[test]
    fn test_getter() {
        let default = TestFullFeaturedStructure::default();

        assert_eq!(&default.age, default.get_age());
        assert_eq!(default.nick, default.get_nick());
        assert_eq!(&default.languages, default.get_languages());
        assert_eq!(&default.hobby, default.get_hobby());
    }

    #[test]
    fn test_getter_mut() {
        let mut default = TestFullFeaturedStructure::default();
        *default.get_age_mut() = 10;
        *default.get_nick_mut() = "another";
        *default.get_languages_mut() = vec!["python".to_string()];
        *default.get_hobby_mut() = Box::new("tennis");

        let copy = TestFullFeaturedStructure::default();

        assert_ne!(copy.age, default.age);
        assert_ne!(copy.nick, default.nick);
        assert_ne!(copy.languages, default.languages);
        assert_ne!(copy.hobby, default.hobby);
    }

    #[test]
    fn test_setter() {
        let mut data = TestFullFeaturedStructure {
            age: Default::default(),
            nick: Default::default(),
            languages: Default::default(),
            hobby: Default::default(),
        };

        let default = TestFullFeaturedStructure::default();
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
        let mut data = TestFullFeaturedStructure::new_default();

        let default = TestFullFeaturedStructure::default();
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
        let default = TestFullFeaturedStructure::default();
        let clone_default = default.clone();

        let data = TestFullFeaturedStructure::new(
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
        let default = TestFullFeaturedStructure::default();
        let mut adjusted_default = default.clone();

        assert!(&default.equals(&adjusted_default));
        assert_eq!(&default.hash_code(), &adjusted_default.hash_code());

        adjusted_default.age = 10;

        assert!(!&default.equals(&adjusted_default));
        assert_ne!(&default.hash_code(), &adjusted_default.hash_code());
    }

    #[test]
    fn test_to_string() {
        let default = TestFullFeaturedStructure::default();
        let string = default.to_string();
        assert_eq!(
            String::from(
                "TestFullFeaturedStructure { age: 0, nick: \"\", languages: [], hobby: \"\" }"
            ),
            string
        )
    }

    #[test]
    fn test_builder() {
        let default = TestFullFeaturedStructure::default();
        let clone_default = default.clone();

        let data = TestFullFeaturedStructure::builder()
            .age(clone_default.age)
            .nick(clone_default.nick)
            .languages(clone_default.languages)
            .hobby(clone_default.hobby)
            .build();

        assert_eq!(&default, &data);
    }

    #[test]
    fn test_data() {
        let mut data = TestDataStructure::new(28, "sokomishalov");

        data.set_age(29);

        assert_eq!(29, *data.get_age());
        assert_eq!("sokomishalov", data.get_nick());
        assert_eq!(
            String::from("TestDataStructure { age: 29, nick: \"sokomishalov\" }"),
            data.to_string()
        )
    }

    #[test]
    fn test_value() {
        let value = TestValueStructure::new(28, "sokomishalov");

        assert_eq!(28, *value.get_age());
        assert_eq!(
            String::from("TestValueStructure { age: 28, nick: \"sokomishalov\" }"),
            value.to_string()
        )
    }
}
