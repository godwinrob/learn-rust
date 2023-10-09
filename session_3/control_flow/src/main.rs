fn main() {

    let x = "Hello, world!";
    let y = long_short(x);
    println!("{y}");

    let x = "hi";
    let y = long_short(x);
    println!("{y}");

    let x: bool = true;
    bool_test(x);
}

fn long_short(x: &str) -> &str {
    if x.len() > 4 {
        "Long word!"
    } else {
        "Short word!"
    }
}

fn bool_test(x: bool) {
    if x {
        println!("Its true!");
    } else {
        println!("Nope!");
    }
}
