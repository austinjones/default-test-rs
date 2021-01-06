use default_test::DefaultTest;

#[derive(Debug, Eq, PartialEq)]
pub struct User {
    id: usize,
    name: String,
    email: String,
    admin: bool,
}

impl DefaultTest for User {
    fn default_test() -> Self {
        Self {
            id: 0,
            name: "name".into(),
            email: "email".into(),
            admin: false,
        }
    }
}

pub fn main() {
    let user = User {
        admin: true,
        ..User::default_test()
    };

    let expected = User {
        id: 0,
        name: "name".into(),
        email: "email".into(),
        admin: true,
    };

    assert_eq!(expected, user);
}
