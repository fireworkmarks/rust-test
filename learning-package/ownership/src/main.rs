fn main() {
    let s1 = String::from("Hello");

    /*
        If use s1 not have clone, IDE will tell you panic. It ownership will change to s2, so s1
        will be gc.

        let s2 = s1; wrong!
     */
    let s2 = s1.clone();

    println!("{}, {}!", s1, s2);

    /*
         Here not need memory copy, they are basic type.
     */
    let a1 = 3;
    let a2 = a1;

    println!("{}, {}!", a1, a2);

    let s = String::from("Hello world!");
    take_ownership(s);
    /*
        here if you want to print s, it will be panic because s has no ownership!
        println!("s: {}", s);
     */

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);
}

fn take_ownership(some_sthing: String){
    println!("{}", some_sthing)
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number)
}