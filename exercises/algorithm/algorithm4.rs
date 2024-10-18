/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Display,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root=Some(Box::new(TreeNode::<T>::new(value)));
        }
        else {
            self.root.as_mut().unwrap().insert(value)
        }
    }

    // Search for a value in the BST
    fn search(&mut self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            false
        }
        else {
            let mut node = self.root.as_mut().unwrap();
            if node.value == value {
                return true
            }
            loop {
                if node.value==value {
                    return true;
                }
                else if value<node.value {
                    if let Some(left) = node.left.as_mut() {
                        node = left;
                    }
                    else {
                        return false;
                    }
                }
                else {
                    if let Some(right) = node.right.as_mut() {
                        node = right;
                    }
                    else {
                        return false;
                    }
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value < self.value {
            if self.left.is_none() {
                let node = Box::new(TreeNode::<T>::new(value));
                self.left = Some(node);
            }
            else {
                self.left.as_mut().unwrap().insert(value);
            }
        }
        else if value > self.value {
            if self.right.is_none() {
                let node = Box::new(TreeNode::<T>::new(value));
                self.right = Some(node);
            }
            else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);


        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


