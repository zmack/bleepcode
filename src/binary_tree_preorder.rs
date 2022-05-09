// Definition for a binary tree node.
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{LinkedList, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: NodeReference,
    pub right: NodeReference,
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

trait PrintableTree {
    fn level_order_traversal(self) -> Vec<Option<i32>>;
}

type NodeReference = Option<Rc<RefCell<TreeNode>>>;

impl PrintableTree for NodeReference {
    fn level_order_traversal(self) -> Vec<Option<i32>> {
        let mut buffer: LinkedList<NodeReference> = LinkedList::new();
        let mut acc: Vec<Option<i32>> = Vec::new();
        buffer.push_back(self);

        while let Some(node) = buffer.pop_front() {
            if node.is_none() {
                acc.push(None);
                continue;
            }

            let cloned_node = node.unwrap().clone();
            let borrowed_node: &RefCell<TreeNode> = cloned_node.borrow();
            let actual_node = borrowed_node.borrow();
            acc.push(Some(actual_node.val));

            if actual_node.left.is_some() || actual_node.right.is_some() {
                buffer.push_back(actual_node.left.clone());
                buffer.push_back(actual_node.right.clone());
            }
        }

        acc
    }
}

#[inline]
fn is_between(value: i32, min: &Option<i32>, max: &Option<i32>) -> bool {
    min.map(|m| value > m).unwrap_or(true) && max.map(|m| value < m).unwrap_or(true)
}

pub fn bst_from_preorder(preorder: Vec<i32>) -> NodeReference {
    fn bst_from_preorder_inner(
        preorder: &mut VecDeque<i32>,
        min_value: Option<i32>,
        max_value: Option<i32>,
    ) -> NodeReference {
        match preorder.pop_front() {
            Some(head) => {
                let mut node = TreeNode::new(head);
                if let Some(&next) = preorder.front() {
                    if is_between(next, &min_value, &max_value) {
                        if next > head {
                            node.right = bst_from_preorder_inner(preorder, Some(head), max_value)
                        } else {
                            node.left = bst_from_preorder_inner(preorder, min_value, Some(head))
                        }
                    }
                }

                if let Some(&next) = preorder.front() {
                    if is_between(next, &min_value, &max_value) && next > head {
                        node.right = bst_from_preorder_inner(preorder, Some(node.val), max_value)
                    }
                }

                Some(Rc::new(RefCell::new(node)))
            }
            None => None,
        }
    }

    let mut prequeue = VecDeque::from(preorder);

    match prequeue.pop_front() {
        Some(head) => {
            let mut node = TreeNode::new(head);
            if let Some(&next) = prequeue.front() {
                if next > head {
                    node.right = bst_from_preorder_inner(&mut prequeue, Some(node.val), None)
                } else {
                    node.left = bst_from_preorder_inner(&mut prequeue, None, Some(node.val))
                }
            }

            if let Some(&next) = prequeue.front() {
                if next > head {
                    node.right = bst_from_preorder_inner(&mut prequeue, Some(node.val), None)
                }
            }

            Some(Rc::new(RefCell::new(node)))
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap_node(node: TreeNode) -> NodeReference {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn level_order_traversal_works() {
        let root: NodeReference = wrap_node(TreeNode {
            val: 1,
            left: wrap_node(TreeNode {
                val: 2,
                left: wrap_node(TreeNode::new(4)),
                right: wrap_node(TreeNode::new(5)),
            }),
            right: wrap_node(TreeNode {
                val: 3,
                left: wrap_node(TreeNode::new(6)),
                right: wrap_node(TreeNode::new(7)),
            }),
        });
        assert_eq!(
            root.level_order_traversal(),
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7)
            ]
        );
    }

    #[test]
    fn level_order_traversal_deals_with_none() {
        let root: NodeReference = wrap_node(TreeNode {
            val: 1,
            left: wrap_node(TreeNode {
                val: 2,
                left: wrap_node(TreeNode::new(4)),
                right: wrap_node(TreeNode::new(5)),
            }),
            right: wrap_node(TreeNode {
                val: 3,
                left: None,
                right: wrap_node(TreeNode::new(7)),
            }),
        });
        assert_eq!(
            root.level_order_traversal(),
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)]
        );
    }

    #[test]
    fn it_actually_does_things() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let node = bst_from_preorder(preorder);
        let expected_output = vec![Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)];

        assert_eq!(expected_output, node.level_order_traversal());
    }

    #[test]
    fn second_test_works() {
        let preorder = vec![1, 3];
        let node = bst_from_preorder(preorder);
        let expected_output = vec![Some(1), None, Some(3)];

        assert_eq!(expected_output, node.level_order_traversal())
    }

    #[test]
    fn third_test_works() {
        let preorder = vec![2, 8, 6, 19];
        let node = bst_from_preorder(preorder);
        let expected_output = vec![Some(2), None, Some(8), Some(6), Some(19)];

        assert_eq!(expected_output, node.level_order_traversal())
    }

    #[test]
    fn fourth_test_works() {
        let preorder = vec![19, 4, 8, 11];
        let node = bst_from_preorder(preorder);
        let expected_output = vec![Some(19), Some(4), None, None, Some(8), None, Some(11)];

        assert_eq!(expected_output, node.level_order_traversal())
    }
}
