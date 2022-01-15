fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s1 = data.to_string();

    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    let mut s4 = String::from("foo");
    s4.push_str("bar");

    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s6 is {}", s6);

    let mut s7 = String::from("lo");
    let c = 'l';
    s7.push(c);
    println!("c is {}", c);

    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let s10 = s8 + &s9; // note s8 has been moved here and can no longer be used

    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");

    let s15 = format!("{}-{}-{}", s11, s12, s13);
    println!("s15 is {}", s15);

    let s14 = s11 + "-" + &s12 + "-" + &s13; // s11 gets borrowed here
    println!("s14 is {}", s14);

    let hello = "Здравствуйте"; // string that uses 2 bytes per character
    let s16 = &hello[0..4];
    println!("s16 is {}", s16);

    // let s17 = &hello[0..1]; // will panic at runtime because we're trying to return less than a character

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
