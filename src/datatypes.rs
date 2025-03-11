//Primitive data types: (int,float,bool,char)

pub fn datatypes() {
//Integer data type:-
//Rust has signed (+ and -) and unsigned integer (only+) types of different types.
//i8,i16,i32,i64,i128 - signed integers and u8,u16,u32,u64,u128 - unsigned integers.
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer:{}",x);
    println!("Unsigned Integer:{}",y);
//diff bet i32(32bits) and i64(64bits)
//range of i32  = 2147483647 and i64  = 9223372036854775807
    let e: i32  = 2147483647;
    let i: i64  = 9223372036854775807;
    println!("Maximum value of i32:{}",e);
    println!("Maximum value of i64:{}",i);
//Floating point data types:-
//f32,f64 are the two floating point numbers.
    let pi: f64 = 3.14;
    println!("Print the numbers:{}",pi);
//Boolean values:-
//true or false
    let is_snowing: bool = true;
    println!("It is snowing? {}",is_snowing);
//Character data types:-
    let letter: char = 'a';
    println!("First letter of the alphabet:{}",letter);
}




