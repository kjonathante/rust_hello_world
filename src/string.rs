pub fn about_strings() {
    // empty mutable String
    let mut s = String::new();

    // to_string converts str to String
    // example #1
    let data = "initial contents";
    let t = data.to_string();
    // example #2
    let u = "Hello".to_string();

    // String::from
    let v = String::from("initial contents");

    let s2 = "bar";
    s.push_str(s2); // push str
    s.push('🚗'); // push char
    println!("{}", s);

    // String concat
    let st1 = String::from("Hello, ");
    let st2 = String::from("world!");
    // note st1 has been moved here and can no longer be used
    // note the & on st2
    let st3 = st1 + &st2;
    println!("{}", st3);

    // String concat that's better than +
    let z = format!("{} {} {}", t, u, v);
    println!("{}", z);

    // Iterating on String
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
