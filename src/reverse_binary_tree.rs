use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    // This is magic version
    #[allow(dead_code)]
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut reverse_nodes = VecDeque::new();

        match &root {
            Some(r) => {
                reverse_nodes.push_back(r.clone());
            }
            None => {
                return None;
            }
        }

        while let Some(node) = reverse_nodes.pop_front() {
            let left_child = node.borrow().left.as_ref().map(|l| l.clone());
            let right_child = node.borrow().right.as_ref().map(|r| r.clone());
            node.borrow_mut().left = right_child.clone();
            node.borrow_mut().right = left_child.clone();
            if left_child.is_some() {
                reverse_nodes.push_back(left_child.unwrap().clone());
            }
            if right_child.is_some() {
                reverse_nodes.push_back(right_child.unwrap().clone());
            }
        }

        root
    }

    #[allow(dead_code)]
    pub fn invert_tree2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parent_nodes = Vec::new();
        let mut child_nodes = Vec::new();
        match &root {
            Some(r_node) => {
                parent_nodes.push(r_node.clone());
            }
            None => {
                return None;
            }
        }

        loop {
            for parent_node in parent_nodes.iter().rev() {
                child_nodes.push(parent_node.borrow().left.clone());
                child_nodes.push(parent_node.borrow().right.clone());
            }

            // reinsert child nodes into parent container
            let child_len = child_nodes.len();
            for i in 0..child_len {
                let left = i;
                let right = child_len - 1 - i;
                if left >= right || !child_nodes[left].is_some() {
                    break;
                }
                // TODO(me): Do we need to check if child node have to be some?
                // switch mapping node
                if child_nodes[right].is_some() {
                    let left_parent = &parent_nodes[left / 2];
                    let right_parent = &parent_nodes[right / 2];

                    if left % 2 == 0 {
                        (*left_parent.borrow_mut()).left = child_nodes[right].clone();
                        (*right_parent.borrow_mut()).right = child_nodes[left].clone();
                    } else {
                        (*left_parent.borrow_mut()).right = child_nodes[right].clone();
                        (*right_parent.borrow_mut()).left = child_nodes[left].clone();
                    }
                }
            }

            if !child_nodes[child_len - 1].as_ref().is_some() {
                break;
            }

            // make current child node as parent nodes
            parent_nodes.clear();
            for child_node in child_nodes.into_iter().rev() {
                parent_nodes.push(child_node.unwrap().clone());
            }
            child_nodes = Vec::new();
        }

        return root;
    }
}

#[test]
fn test_reverse_binary_tree() {
    let root = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        }))),
    }));

    // let mut s = Solution;
    Solution::invert_tree(Some(root));
}
