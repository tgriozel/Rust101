fn main() {
    let n = 666;
    makes_copy(n);
    println!("will try to access again: {}", n);

    let s = String::from("s");
    let s_clone = s.clone();
    println!("both {} and its clone {} are accessible", s, s_clone);
    takes_ownership(s);
    //println!("will try to access again: {}", s); -> does not work!
    let s2 = gives_ownership();
    println!("will try to access again: {}", s2);
    let s3 = takes_and_gives_back_ownership(String::from("s3"));
    println!("will try to access again: {}", s3);
}

fn makes_copy(an_integer: i32) {
    println!("makes copy: {}", an_integer);
}

fn takes_ownership(a_string: String) {
    println!("takes ownership: {}", a_string)
}

fn gives_ownership() -> String {
    let s = String::from("here");
    s
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    println!("takes_and_gives_back_ownership: {}", a_string);
    a_string
}
