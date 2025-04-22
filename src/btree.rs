pub struct BPTree<K, V> {
    root: Node<K, V>,
    length: usize,
    depth: usize,
    b: usize,
}

struct InteriorNode<K, V> {
    keys: Vec<K>,
    edges: Vec<Node<K, V>>,
}

struct LeafNode<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

enum Node<K, V> {
    Leaf(LeafNode<K, V>),
    Interior(InteriorNode<K, V>),
}

impl<K, V> BPTree<K, V> {
    fn new() -> BPTree<K, V> {
        BPTree {
            root: Node::Leaf(LeafNode::new()),
            length: 0,
            depth: 0,
            b: 0,
        }
    }

    fn find(&self, key: &K) -> Option<&V> {
        let cur_node = &self.root;

        None
    }
}

impl<K, V> LeafNode<K, V> {
    fn new() -> Self {
        LeafNode {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    fn leaf_search(&self, key: K) -> Option<&V> {
        todo!();
    }
}
impl<K, V> InteriorNode<K, V> {
    fn new() -> Self {
        InteriorNode {
            keys: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_btree() {
        let map: BPTree<i32, i32> = BPTree::new();
    }
}
