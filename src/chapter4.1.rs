fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    let z = 10;

    println!("Success!");
}


fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}


fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}


fn main() {
   let v1 = 251_u8.wrapping_add(8);
   let v2 = i8::checked_add(251, 8).unwrap_or(-1);

   println!("{},{}", v1, v2);
}


fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}


fn main() {
    let x = 1_000.000_1; // f64 by default
    let y: f32 = 0.12;   // f32
    let z = 0.01_f64;    // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


fn main() {
    let epsilon = 1e-10; // Define a small tolerance
    assert!((0.1 + 0.2 - 0.3).abs() < epsilon);

    println!("Success!");
}


fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}


use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}



fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1u8 as i8 - 2 == -1);
    assert!(3 * 50 == 150);
    assert!((9.6 / 3.2 - 3.0).abs() < f32::EPSILON);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
