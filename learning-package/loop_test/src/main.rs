fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter + 5;
        }
    };

    println!("The result is {}", result);
}
