// Functions
fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    // Immutables //
    let x: f64 = 0.00000000000000003;
    let y: f64 = 0.00000000000000001;

    let z: f64 = add(x.into(), y);

    println!("The sum of x:{} and y: {} is {}", x, y, z);

    // Mutable //

    // String literals
    let mut name: &str = "Axel";
    println!("Name before changing: {}", name);

    name = "Axelin";
    println!("Name after changing: {}", name);

    let last_name: String = "Galicia".to_string();
    println!("Last Name: {}", last_name);

    // String slice (pointer to the begining and pointer to the end of the string)
    let name_slice: &str = &name;

    println!("Printing slice value: {}", name_slice);

    // Vector/arrays //

    // Fixed-size array
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    println!("Printing fixed-size array {:?}", four_ints);

    // A dynamic array
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    println!("Printing vector: {:?}", vector);

    // Tuples //
    let m: (i32, &str, f64) = (1, "Axel", 7.9);

    // Destructuring //

    let (a, b, c) = m;
    println!("{} {} {}", a, b, c);

    // Indexing //
    println!("{}", m.1);

    // Types //
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };
    println!("{:?}", origin);

    // Tuple struct with unnamed fields //
    #[derive(Debug)]
    struct Point2(i32, i32);
    let origin2 = Point2(0, 0);
    println!("{:#?}", origin2);

    // Basic C-Like enum

    #[derive(Debug)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;
    println!("{:?}", up);

    // Enum with fields //
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing: OptionalI32 = OptionalI32::Nothing;

    // Generics //

    #[derive(Debug)]
    struct Person<T> {
        property: T,
    }

    // This is alreadu defined in the standard library as `Option`
    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    // Methods //
    impl<T> Person<T> {
        fn bar(&self) -> &T {
            // Self is borrowed
            &self.property
        }

        fn bar_mut(&mut self) -> &mut T {
            // self is mutably borrowed
            &mut self.property
        }

        fn into_bar(self) -> T {
            // here self is consumed
            self.property
        }
    }

    let mut person_one: Person<&str> = Person { property: "Axel" };
    person_one.property = "Zoe";
    println!("{:?}", person_one);

    // Wrap Person to implement Display Trait
    struct PersonPrinter<T> {
        person: Person<T>,
    }

    impl<T> std::fmt::Display for PersonPrinter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "properties")?;
            write!(f, "Done")
        }
    }

    // Traits (known as interfaces or typeclasses in other languages) //

    trait Greeter<T> {
        fn greet(self) -> Option<T>;
    }

    impl<T> Greeter<T> for PersonPrinter<T> {
        fn greet(self) -> Option<T> {
            Some(self.person.property)
        }
    }

    let z: PersonPrinter<&str> = PersonPrinter {
        person: { Person { property: "Axel" } },
    };

    println!("{:?}", z.greet());

    // Function pointer types //

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    type FunctionPointer = fn(u32) -> u32;

    let fib: FunctionPointer = fibonacci;

    println!("Fib: {}", fib(4));

    // Pattern Matching //
    let optional_i32 = OptionalI32::AnI32(4);

    match optional_i32 {
        OptionalI32::AnI32(n) => println!("It's an i32 {}", n),
        OptionalI32::Nothing => println!("It's nothing!"),
    }

    // Advanced pattern matching //
    struct Coordinates {
        x: i32,
        y: OptionalI32,
    }

    let mexico_city_coordinates: Coordinates = Coordinates {
        x: 15,
        y: OptionalI32::AnI32(32),
    };

    match mexico_city_coordinates {
        Coordinates {
            x: 0,
            y: OptionalI32::AnI32(0),
        } => println!("The numbers are zero!"),
        Coordinates {
            x: n,
            y: OptionalI32::AnI32(m),
        } if n == m => println!("Numbers are the same"),
        Coordinates {
            x: n,
            y: OptionalI32::AnI32(m),
        } => println!("Different numbers {} {}", n, m),
        Coordinates {
            x: _,
            y: OptionalI32::Nothing,
        } => println!("The second number is Nothing!"),
    }

    // Control flow //

    // `for` loops/iteration
    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    // Ranges
    for i in 0u32..10 {
        println!("{}", i);
    }

    if 1 == 1 {
        println!("Math is working");
    } else {
        println!("Oh no");
    }

    while 2 == 2 {
        println!("Counting once");
        break;
    }

    let mut counter: i32 = 0;

    // Infinite loop
    loop {
        println!("Hello!");
        if counter == 4 {
            break;
        }
        counter += 1;
    }

    // Memory safety and pointers //
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference

    println!("Mine box: {}", mine);
    let mut now_its_mine = mine; // now_its_mine takes ownership of mine, in other words, mine is moved
    *now_its_mine += 10;
    println!("Mine after move: {}", now_its_mine);

    // Reference - an immutable pointer that refers other data
    // When a reference is taken to a value, we say that the value has been `borrowed`
    // While a value is borrowed immutably, it cannot be mutated or moved.
    // A borrow is active until the last use of the borrowing variable
    let mut var = 4;
    let ref_var: &i32 = &var;

    println!("{}", var); // Unlike mine, var can still be used
    println!("{}", *ref_var);

    // Mutable reference
    // While a value is mutably borrowed, it cannot be accessed at all.
    let mut var2 = 7;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 10;

}
