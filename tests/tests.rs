#[cfg(test)]
mod tests {
    use lombok_rs::Getter;

    #[derive(Getter)]
    pub struct TestGetter {
        age: u8,
        nick: &'static str,
        position: String,
    }

    #[test]
    fn test_getters() {
        let data = TestGetter {
            age: 25,
            nick: "sokomishalov",
            position: "developer".to_string(),
        };

        assert_eq!(25, data.age);
        assert_eq!("sokomishalov", data.nick);
        assert_eq!("developer", data.position);
    }
}
