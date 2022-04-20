fn main() {
    mutability();
    data_types();
    functions();
    control_flow();
}

fn mutability() {
    println!("\n#mutability");
    let mut x = 1;
    println!("{}", x);
    x = 4;
    println!("{}", x);

    // type annotation required
    // &str, borrowed form??
    const HELLO_WORLD: &str  = "hello world!";
    println!("{}", HELLO_WORLD);

    // shadow
    let a = 5;
    let a = a + 1;
    println!("{}", a);

    // shadowing is bound to scope
    {
        let a = a + 3;
        println!("{}", a)
    }

    println!("{}", &a) // a vs &a?
}

fn data_types() {
    println!("\n#data_types");
    let x = 2;
    let y = 4;
    println!("{}", x / y);

    // 4 bytes
    let char_1 = 'a';

    let tup_1 = (1, 2.3, 'a');
    let tup_2: (i32, i64, i8) = (10, 10, 10);
    let sum = tup_2.0 + tup_1.0;

    // unit
    let empty = ();

    // Array: consists of same type. fixed length
    // allocated on stack, not heap
    let arr_1 = [1, 2, 3];
    let threes = [3;3];

    // Invalid array index access will panic
}

fn functions() {
    println!("\n#functions");

    // Statement vs Experession

    // Statement does some action. Does not returns value
    // let x = (let y = 3);  -> error: expected expression, found statement (`let`)

    // Expression evaluates to a value
    let y = {
        let x = 3;
        x + 1 // <- expression
    };
    println!("{}", y)
}

fn control_flow() {
    println!("\n#control flows");
    if true {
        let x = if true { 1 } else { 100 };
        println!("{}", x);
    }

    let arr = [3;3];
    for el in arr {
        println!("{}", el)
    }

    for x in -1..1 {
        println!("{}", x);
    }
}