
pub fn demo() {
    println!("Types demo");
    let a = 'c';    //char variable
    let b: i32 = 0;         //defines integer 32 (32 bits)
    let c: bool = false;    //boolean variable
    let d: &'static str = "Hello World"; //String or &'static str
    let e: String = "Hello World".to_string();
    let f = [1, 2, 3];      //array
    let g = [0; 5];        //create a 5 size array and init all to 0
    let h = (1, 2, 'h', "Hello");
    println!("Char {}", a);
    println!("Int {}", b);
    println!("Bool {}", c);
    println!("String {}", d);
    println!("String 2 {}", e);
    println!("Array {:?}", f);
    println!("Array {:?}", g);
    println!("Tupple {:?}", h);
}
