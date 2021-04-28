use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    i: i64,
    x: f32,
    y: f32
}

fn main() {
    // let x: i32 = 5;
    // let xs: Vec<u8> = bincode::serialize(&x).unwrap();
    // println!("i32 number {} serializes into byte array {:?}", x, xs);
    // let xd: i32 = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let x: f32 = 3.14;
    // let xs = bincode::serialize(&x).unwrap();
    // println!("f32 number {} serializes into byte array {:?} with count {:?}", x, xs, xs.len());

    // let x: Vec<u8> = vec![1, 2, 3];
    // let xs = bincode::serialize(&x).unwrap();
    // println!("Vec<u8> {:?} serializes into byte array {:?}", x, xs);
    // let xd: Vec<u8> = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    // let xs = bincode::serialize(&x).unwrap();
    // println!("Vec<f32> {:?} serializes into byte array {:?} with count {:?}", x, xs, xs.len());
    // let xd: Vec<f32> = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let x: Vec<f32> = vec![3.14];
    // let xs = bincode::serialize(&x).unwrap();
    // println!("Vec<f32> {:?} serializes into byte array {:?} with count {:?}", x, xs, xs.len());
    // let xd: Vec<f32> = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    // let xs = bincode::serialize(&x).unwrap();
    // println!("tuple {:?} serializes into byte array {:?}", x, xs);
    // let xd: (i32, &str, f32, bool) = bincode::deserialize(&xs).unwrap();
    // assert_eq!(x, xd);

    // let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    // let xs = bincode::serialize(&x).unwrap();
    // println!("nested tuple {:?} serializes into byte array {:?}", x, xs);

    let point1: Point = Point {i:1, x:1.0, y:2.0};
    let point2: Point = Point {i:1, x:3.0, y:4.0};
    let point1s = bincode::serialize(&point1).unwrap();
    let point2s = bincode::serialize(&point2).unwrap();
    println!("struct Point serializes into byte array {:?} count: {:?}", point1s, point1s.len() );
    println!("struct Point serializes into byte array {:?}", point2s);
}
