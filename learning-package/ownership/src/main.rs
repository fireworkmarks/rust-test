fn main() {
    let s1 = String::from("Hello");

    /*
        if use s1 not have clone, IDE will tell you panic. It ownership will change to s2, so s1
        will be gc.

        let s2 = s1; wrong!
     */
    let s2 = s1.clone();

    println!("{}, {}!", s1, s2)
}
