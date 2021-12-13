// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
use serde::{Serialize, Deserialize};

// pub enum Node {
//     // Internal(Internal),
//     Leaf {
//         data: Leaf,
//         parent: RefCell<Weak<Node>>
//     }
// }

// pub struct Internal {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaf {
    pub is_root: bool,
    pub rows: Vec<(u32, Vec<u8>)>
}

#[cfg(test)]
mod tests {
    use bincode;
    use crate::btree::nodes::Leaf;
    use crate::table::{row_util, Row};

    fn init_mock_rows() -> Vec<(u32, Vec<u8>)> {
        let r1 = Row::new(1, "test_user", "test_user@email.com").unwrap();
        let r2 = Row::new(2, "test_user_2", "test_user_2@email.com").unwrap();

        return vec![
            (r2.id, row_util::serialize_row(&r2)),
            (r1.id, row_util::serialize_row(&r1))
        ];
    }

    fn build_leaf_node(rows: Vec<(u32, Vec<u8>)>) -> Leaf {
        Leaf { is_root: true, rows }
    }

    #[test]
    fn test_leaf_serialization() {
        let rows = init_mock_rows();
        let leaf = build_leaf_node(rows);
        let serialized_leaf = bincode::serialize(&leaf).expect("Failed to serialize leaf");
        let deserialized_leaf: Leaf = bincode::deserialize(&serialized_leaf).expect("Failed to deserialize leaf");

        assert_eq!(deserialized_leaf.rows[0].0, leaf.rows[0].0);
        assert_eq!(deserialized_leaf.rows[0].1, leaf.rows[0].1);

        assert_eq!(deserialized_leaf.rows[1].0, leaf.rows[1].0);
        assert_eq!(deserialized_leaf.rows[1].1, leaf.rows[1].1);
    }
}

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }
//
// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });
//
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//
//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
//
//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
// }


