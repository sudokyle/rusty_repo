use crate::reference_binary_tree::BinaryTree;

mod sorting;
mod reference_binary_tree;


fn main() {
    let mut binary_tree = BinaryTree::new();
    binary_tree.insert(10);
    binary_tree.insert(20);
    binary_tree.insert(5);
    binary_tree.insert(15);
    binary_tree.insert(12);
    binary_tree.insert(3);
    binary_tree.insert(2);
    binary_tree.insert(25);
    binary_tree.in_order_walk();
    println!("##############################");
    binary_tree.delete(&20);
    binary_tree.in_order_walk();
    println!("##############################");

    let mut unordered_list = vec![5, 3, 10, 12, 6];
    println!("Unsorted: {:?}", unordered_list);
    sorting::merge_sort(&mut unordered_list);
    println!("Sorted: {:?}", unordered_list);
}
