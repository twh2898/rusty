extern crate time;

macro_rules! log {
    ($fmt:expr) => (print!("[{}][LOG]{}\n", time::strftime("YYYY-MM-DD HH:MM:SS UTC", &time::now_utc())
        .unwrap_or_else(|e| panic!("The time format failed. {}", e)), $fmt));
    ($fmt:expr, $($arg:tt)*) => (print!("[{:?}][LOG]{}", time::now(), "[LOG]", concat!($fmt, "\n"), $($arg)*));
}

pub fn demo() {
    println!("{}",
             time::strftime("%Y-%m-%d %H:%M:%S UTC", &time::now_utc())
                 .unwrap_or_else(|e| panic!("The time format failed. {}", e)));
    println!("Formatting demo");
    let a = format!("{arg}", arg = "test");   //format! print! println! are interchangeable
    println!("{}", a);
    println!("{name} {}", 1, name = 2); // => 2 1
    println!("{a} {c} {b}", a = 'a', b = 'b', c = 3);   // => a c 3
    // println!("{0:X} {0:o}", 10); //Invalid, can't refear to an argument as multiple types
    let fmt_num = format!("{:.*}", 2, 1.234567);
    assert_eq!("1.23", fmt_num);
    log!("Hello World form Log");
    //  Formatting Traits
    //  nothing => Display
    //  ? => Debug
    //  o => Octal
    //  x => Lower Hex
    //  X => Upper Hex
    //  p => Pointer
    //  b => Binary
    //  e => lower Exp (1.45e3)
    //  E => Upper Exp (1.45E3)
    //

}
