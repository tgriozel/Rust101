fn main() {
    let mut s = String::from("s");
    just_reading(&s);
    println!("{} is still in original scope", s);
    change_str(&mut s);
    println!("changing: {}", s);
}

fn just_reading(a_string: & String) {
    println!("just reading {}", a_string)
}

fn change_str(a_string: &mut String) {
    a_string.push_str("_appended");
}

// fn dangle() -> & String { -> compiler not happy!
//     let s = String::from("dangling");
//     &s
// }
