fn main() {
    println!("Hello world!");
    another_function(31);
    let five = five();
    let five = if five == 5 { 4 } else { 5 };
    println!("Five: {five}");
    if five == 5 {
        println!("The five is a five.");
    } else {
        println!("The five is not a five.");
    }
}

fn another_function(n: i32) {
    println!("Value of arg: {n}");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}
