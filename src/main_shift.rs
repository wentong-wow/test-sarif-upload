fn foo(n: i32) -> i32 {
    n + 1
}

fn main() {
    // Absolute path lint issue: use an absolute path for standard library reference
    let _v: ::std::vec::Vec<i32> = vec![1, 2, 3];

    // Absurd extreme comparison lint issue: comparing a value with an extreme that will always be true or false
    let x = 10;
    if x < i32::MIN {
        println!("This will never print, but the lint will be triggered.");
    }

    let _ = foo(x);

    // Almost complete range lint issue: creating a range that is very close to being complete
    let _ = 'a'..'z';
}