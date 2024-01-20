mod sorting;

fn main() {
    let mut unordered_list = vec![5, 3, 10, 12, 6];
    println!("Unsorted: {:?}", unordered_list);
    sorting::merge_sort(&mut unordered_list);
    println!("Sorted: {:?}", unordered_list);
}
