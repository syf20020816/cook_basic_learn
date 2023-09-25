enum TreeNode {
    Root = 2,
    Branch = 1,
    Leaf = 0,
}

enum Page{
    Container = 1,
    Box = 10,
    Component = 100,
    Element = 0
}

impl Node {
    pub fn as_num(self) -> isize {
        self as isize
    }
    pub fn from_num(num: u8) -> Node {
        match num {
            0 => Node::Node1,
            _ => Node::Node2,
        }
    }
}

fn main() {
    dbg!(Node::Node3.as_num());

}