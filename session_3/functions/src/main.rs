fn main() {
    println!("Hello, world!");

    another_one();

    another_some(10);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    let z = five();

    println!("five! {z}");

    let z = plus_one(z);

    println!("six! {z}");
}

fn another_one() {
    println!("Another one!");
}

// Commenting
fn another_some(x: u32) {
    println!("Another {x}!")
}

fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
   x + 1
}
