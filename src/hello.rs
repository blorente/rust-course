fn numbers_and_variables() {
    // Numbers //

    // Immutable bindings
    let x: i32 = 1;

    // Type inference
    // Most of the time, the Rust compiler can infer what type a variable is, so
    // you don’t have to write an explicit type annotation.
    let implicit_x = 1; // Will be an int.
    let implicit_f = 1.3; // Will be a float.

    // Arithmetic
    let y = 2;
    let sum = x + y + 13; // = 16

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;
}

fn string_and_printing() {
    // Strings //

    // String literals
    let x: &str = "hello world!";

    // Printing
    println!("{} {}", 1.3, x); // 1.3 hello world

    // A `String` – a heap-allocated string
    let s: String = "hello world".to_string();

    // String interpolation
    let s: String = format!("I have {} cookies", 2);

    // A string slice – an immutable view into another string
    // This is basically an immutable pair of pointers to a string – it doesn’t
    // actually contain the contents of a string, just a pointer to
    // the begin and a pointer to the end of a string buffer,
    // statically allocated or contained in another object (in this case, `s`)
    let s_slice: &str = &s;

    println!("{} {}", s, s_slice); // hello world hello world
}

fn vectors_arrays_and_tuples() {
    // Vectors/arrays //

    // A fixed-size array
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // A slice – an immutable view into a vector or array
    // This is much like a string slice, but for vectors
    let slice: &[i32] = &vector;

    // Use `{:?}` to print something debug-style
    println!("{:?} {:?}", vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]

    // Tuples //

    // A tuple is a fixed-size set of values of possibly different types
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // Destructuring `let`
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 hello 3.4

    // Indexing
    println!("{}", x.1); // hello
}

fn structs_and_enums() {
    // Struct
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let origin: Point = Point { x: 0, y: 0 };
    }

    // Basic C-like enum
    {
        enum Direction {
            Left,
            Right,
            Up,
            Down,
        }

        let up = Direction::Up;
    }

    // Enum with fields
    {
        enum OptionalI32 {
            AnI32(i32),
            Nothing,
        }

        let two: OptionalI32 = OptionalI32::AnI32(2);
        let nothing = OptionalI32::Nothing;
    }

    // Generics //
    {
        struct Foo<T> { bar: T }
        
        // This is defined in the standard library as `Option`
        enum Optional<T> {
            SomeVal(T),
            NoVal,
        }
    }
}

fn methods() {
    
    // Defining methods for structs //

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        // Methods take an explicit `self` parameter
        fn get_x(self) -> T {
            self.x
        }

        // This can also hold functions
        // Note the lack of `self` parameter
        fn new(x: T, y: T) -> Point<T> {
            Point {
                x: x,
                y: y,
            }
        }
    }
    
    // Calling Methods for structs

    let a_point = Point { x: 1, y: 2 }; // T is resolved to unsigned int.
    let new_point = Point::new(2, 3);
    println!("{}", a_point.get_x()); // 1
}

fn traits() {
    // Traits (known as interfaces or typeclasses in other languages) //
    trait Animal {
        fn name(self) -> String;
        fn noise(self) -> String;
    }

    struct Cat {
        name: String,
        is_hungry: bool
    }

    impl Cat {
        fn new(name: String) -> Self {
            Cat {
                name: name,
                is_hungry: true,
            }
        }

        fn feed(&mut self) {
            self.is_hungry = false
        }
    }

    impl Animal for Cat {
        fn name(self) -> String {
            self.name.clone()
        }

        fn noise(self) -> String {
            if self.is_hungry {
                String::from("Meow!");
            } else {
                String::from("Prrrrr...")
            }
        }
    }
    
    let roger = Cat::new("Roger");
    roger.noise(); // Meow!
    roger.feed();
    roger.noise(); // Prrrrr...
}

fn control_flow() {
    // `for` loops/iteration
    {
        let array = [1, 2, 3];
        for i in array.iter() {
            println!("{}", i);
        }
    }

    // Ranges
    { 
       for i in 0..10 {
            print!("{} ", i);
        }
        println!("");
        // prints `0 1 2 3 4 5 6 7 8 9 `
    }

    // `if`
    {
        if 1 == 1 {
            println!("Maths is working!");
        } else {
            println!("Oh no...");
        }
    }

    // `if` as expression
    let value = if true {
        "good"
    } else {
        "bad"
    }; // value = good

    // pattern matching
    let cookies: u32 = 4;
    match cookies {
        0 => println!("Sadness."),
        1 => println!("It's a low amount of cookies."),
        2|3 => println!("It's an okay amount of cookies, I guess..."),
        4..6 => println!("It's an adequate amount of cookies."),
        _ => println!("It might be too many cookies.")
    }

    // `while` loop
    while 1 == 1 {
        println!("The universe is operating normally.");
        // break statement gets out of the while loop.
        //  It avoids useless iterations.
        break
    }

    // Infinite loop
    loop {
        println!("Hello!");
        // break statement gets out of the loop
        break
    }
}

fn error_handling_patterns() {
    // The Result enum
    enum Result<T, Y> {
        Ok<T>,
        Err<Y>,
    }

    // If we have the following...
    let foo: Result<u32, String> = "42".parse();

    // Simple solutions to simple problems
    let unwrapped_foo: u32 = foo.unwrap(); // Panic on error
    let expected_foo: u32 = foo.expect("Failed to parse number "); // Panic on error, with a nice message!

    // Pattern matching!
    match foo {
        Ok(number) => println!("The number is  {}", number),
        Err(err)  => println!("Error parsing number: {}", err),
    }

    // Functional combinators:
    //  Rust            Scala
    //   `map`       ~= `map`
    //   `and_then`  ~= `flatmap
    fn save_number_to_disk_fallible(num: u32) -> Result<u32, String> {
        unimplemented!()
    }
    
    foo
        .map(|number| number + 2 ) // Operate only if success. Return an item.
        .and_then(|number| save_number_to_disk_fallible(number)) // Operate only if success. Return a Result.
        .and_then(save_number_to_disk_fallible) // The same thing, except we pass a function.
        .map_err(|err| panic!("THis is a vVery bad ERROR omygosh {}", err)) // Operate only in case of error

}

fn hash_derive() {
    // Rust can infer the implementation of some traits
    struct Point {
        x: u32,
        y: u32
    }

    let p = Point {x: 1, y: 2};
    println!("I'm debugging a new point! {:?}", p); // Error!

    #[derive(Debug)]
    struct PrintablePoint {
        x: u32,
        y: u32
    }

    let pp = PrintablePoint {x: 1, y: 2};
    println!("New Point {}", pp); // Error!
}


#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
// Main function
fn main() {

    ///////////////
    // 1. Basics //
    ///////////////

    numbers_and_variables();
    string_and_printing();
    vectors_arrays_and_tuples();
    control_flow();
    error_handling_patterns();

    ////////////////////////////
    // 2. Types & Abstractions//
    ////////////////////////////

    structs_and_enums();
    methods();
    traits();
    hash_derive();
    error_handling_patterns();
}
