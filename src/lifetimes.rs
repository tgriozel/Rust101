use crate::model::person::NamedPerson;

fn oldest<'a>(x: &'a NamedPerson, y: &'a NamedPerson) -> Option<&'a NamedPerson<'a >> {
    let diff = x.age - y.age;
    let res = match diff {
        0 => None,
        d if d < 0 => Some(y),
        _ => Some(x),
    };
    return res;
}

pub fn main() {
    const ALICE: NamedPerson = NamedPerson { name: "Alice", age: 32 };
    const BOB: NamedPerson = NamedPerson { name: "Bob", age: 64 };
    const ALICE_REF: &NamedPerson = &ALICE;
    const BOB_REF: &NamedPerson = &BOB;
    let oldest = oldest(&ALICE, &BOB);
    println!("Among {:?} and {:?} the oldest is...", ALICE, BOB);
    match oldest {
        Some(ALICE_REF) => println!("The dear Alice!"),
        Some(BOB_REF) => println!("The great Bob!"),
        _ => println!("No one!"),
    }
}
