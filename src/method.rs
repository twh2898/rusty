fn foo(bar: i8) {
    println!("Value {}", bar);
}

fn add(a: i32, b: i32) -> i32 {
    a + b   //no semicolin, no return keyword
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b;   //semicolin and return keyword
}

fn div() -> ! {
    panic!("This is a diverging method, No Return!");
}

pub fn demo() {
    println!("Method demo");
    foo(8);
    let c: i32 = add(1, subtract(3, 1));
    println!("2 + 2 = {:}", c);
    div();
}
