fn main() {
    // Variable
    let x = 5;
    println!("value of x: {x}");
    // x = 6;
    // println!("value of x: {x}");

    let mut y = 5;
    println!("value of y: {y}");
    y = 6;
    println!("value of y: {y}");

    // Constant
    const Z: u32 = 20 * 2 * 10;
    println!("value of constant z: {Z}");

    //Shadowing
    let a = 5;
    println!("value of a: {a}");

    let a = a + 1;
    println!("value of a: {a}");

    {
        let a = a * 2;
        println!("value of a: {a}");
    }
    println!("value of a: {a}");

    let b = "    ";
    println!("value of b: {b}");
    // b = b.len();
    let b = b.len();
    println!("value of b: {b}");


    // Datatype
    // let num: i64 = 1_00_00000000;
    // let num: i64 = 0b1_00_00000000;
    let num: i64 = 0xffff;

    println!("num: {num}");

    let num_float = 25.25;
    println!("num_float: {num_float}");

    let is_true = true;
    println!("is_true: {is_true}");

    let ch = 'a';
    println!("char: {ch}");

    // Tuple
    let tup = ('1', 'a', "abc");
    // destructuring
    let (q, w, e) = tup;
    println!("q: {q}, w: {w}, e: {e}");

    // why below line is not working?
    // println!("{tup.0}");

    let mut tup2 = ('1', 'a', "abc");
    tup2.0 = 'c';

    // Array
    let arr = [1, 2, 3, 4];
    println!("{:?}", arr);

    // let arr = ['a', 1, 2, 3, 4];
    // let arr = [1, 2, 3, 4, 'a'];
    
    let arr2 = [3; 5];
    println!("{:?}", arr2);

    // let arr3: [i8; 3] = [3; 5];
    let arr3: [i8; 3] = [1; 3];
    println!("{:?}", arr3);

    let arr4: [i8; 3] = [1, 2, 3];
    println!("{:?}", arr4);
    println!("{}", arr4[0]);
    // 'index out of bounds' is runtime error
    // println!("{}", arr4[5]);

    let new_str = String::new();
    println!("{new_str}");

}
