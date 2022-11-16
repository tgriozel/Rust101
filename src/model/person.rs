#[derive(Debug, Clone, Copy)]
pub struct Person {
    pub age: i8
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamedPerson<'a> {
    pub name: &'a str,
    pub age: i8,
}
