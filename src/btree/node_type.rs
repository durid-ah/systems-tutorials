pub enum NodeType {
   Internal,
   Leaf
}

pub enum Node {
   Internal(Internal),
   Leaf(Leaf)
}

pub struct Internal { }
pub struct Leaf { }