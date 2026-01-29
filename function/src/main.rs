fn main() {
    println!("Hello, world!");

    let x = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {x}");

    another_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function(x:i32) {
    println!("Another function. {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}