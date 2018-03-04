fn main() {
    // Chapter 1 - Introduction
    println!("Hello, world!");

    // Chapter 2 - Guessing Game Tutorial
        // See guessing_game directory

    // Chapter 3 - Common Programming Concepts
    mut_variables();
    shadowing();
    number_literals();
    int_types();
    float_types();
    numeric_operations();
    bool_types();
    character_types();
    tuples();
    arrays();
    another_function(5);
    expressions();
    println!("The value of x is: {}", five());

    // A comment
    //
    // No special multiline comment character. :)
    //

    if_expressions();
    if_expressions_multiple_conditions(2);
    if_expressions_multiple_conditions(4);
    if_expressions_multiple_conditions(5);
    if_expressions_multiple_conditions(6);
    if_in_a_let_statement(true);
    if_in_a_let_statement(false);
    basic_loop();
    while_loop();
    collection_loop();
    iter_loop();

    // Chapter 4 - Understanding Ownership
    ownership();
    ownership_strings();
    cloning();

    let s  = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn mut_variables() {
    println!("Mutable variables:");

    let mut x = 5;
    println!("The value of mut x is: {}", x);

    x = 6;
    println!("The value of mut x is: {}", x);
}

fn number_literals() {
    println!("Number literals:");
    println!("{} {} {} {} {}", 98_222, 0xff, 0o77, 0b1111_0000, b'A');
}

fn int_types() {
    println!("Integer types:");

    let s8 : i8 = i8::max_value();
    let s16 : i16 = i16::max_value();
    let s32 : i32 = i32::max_value();
    let s64 : i64 = i64::max_value();
    let un8 : u8 = u8::max_value();
    let un16 : u16 = u16::max_value();
    let un32 : u32 = u32::max_value();
    let un64 : u64 = u64::max_value();

    println!("{} {} {} {}", s8, s16, s32, s64);
    println!("{} {} {} {}", un8, un16, un32, un64);
}

fn float_types() {
    println!("Float types:");

    let fp32 : f32 = 1.0;
    let fp64 : f64 = 3.14;

    println!("{} {}", fp32, fp64);
}

fn numeric_operations() {
    println!("Numeric operations:");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);
}

fn bool_types() {
    println!("Boolean types:");

    let t = true;
    let f: bool = false;

    println!("{} {}", t, f);
}

fn character_types() {
    println!("Character types");

    let c = 'z';
    let z = 'ℤ';
    let doggo = '🐶';

    println!("{} {} {}", c, z, doggo);
}

fn shadowing() {
    println!("Shadowing:");

    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);
}

fn tuples() {
    println!("Tuples:");

    let tup = (8, 16.0, 32);
    let (x, y, z) = tup;

    println!("{} {} {}", x, y, z);
    println!("{} {} {}", tup.0, tup.1, tup.2);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];

    println!("{}", a[0]);
    println!("{}", a[1]);
    println!("{}", a[2]);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn if_expressions() {
    let number = 7;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }
}

fn if_expressions_multiple_conditions(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_in_a_let_statement(condition: bool) {
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

fn basic_loop() {
    let mut i = 0;

    loop {
        println!("again!");
        i += 1;
        if i > 10 {
            break;
        }
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn collection_loop() {
    let a = [
        "  * *",
        " *   *",
        "* ._. *",
        " *   *",
        "  * *",
        ""
    ];
    let mut index = 0;

    while index < 6 {
        println!("{}", a[index]);

        index = index + 1;
    }
}

fn iter_loop() {
    let a = [
        "  % %",
        " %   %",
        "% ^_^ %",
        " %   %",
        "  % %",
        ""
    ];

    for element in a.iter() {
        println!("{}", element);
    }
}

fn ownership() {
                        // s is not valid here, has not been declared
    let s = "hello";    // s is valid from here on forward

    // do stuff with s
    println!("s, which is {}, is still valid", s);

    // this scope is over, and now is no longer valid
}

fn ownership_strings() {
    // This type of string, the `String` type, is allocated on the
    // heap and mutable, unlike string literals which are placed on
    // the stack and immutable.
    let mut s = String::from("hello");

    s.push_str(" world");

    println!("{}", s);
}

fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.