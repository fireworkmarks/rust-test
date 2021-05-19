fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;
    while index < 6 {
        println!("The value is: {}.", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The value is: {}.", element);
    }
}
