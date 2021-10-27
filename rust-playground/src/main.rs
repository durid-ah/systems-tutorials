use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    i: i64,
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

fn main() {
    // let s = String::from_utf8(vec![b'x'; 255]).unwrap();
    // let s1 = String::from_utf8(vec![b'x'; 32]).unwrap();
    // let r3: Row = Row{id: 1, username: s, email: s1};
    // let r3s = bincode::serialize(&r3).unwrap();
    // println!("Struct size: {:?}", r3s.len());
    // // println!("Struct content: {:?}", r3s);

    // let s = String::from_utf8(vec![b'x'; 1]).unwrap();
    // let s1 = String::from_utf8(vec![b'x'; 1]).unwrap();
    // let r4: Row = Row{id: 1, username: s, email: s1};
    // let r4s = bincode::serialize(&r4).unwrap();
    // println!("Struct size: {:?}", r4s.len());
    // println!("Struct content: {:?}", r4s);

    let mut s: Vec<i32> = Vec::with_capacity(307);
    s.push(1);
    println!("{:?}", s);
    println!("{:?}", s.capacity());
    println!("{:?}", s.len());
}
