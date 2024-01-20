
pub fn merge_sort(a: &mut Vec<i32>) {
    inner_merge_sort(a, 0, a.len()-1);
}

fn inner_merge_sort(a: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let middle = (start + end) / 2;
        inner_merge_sort(a,start, middle);
        inner_merge_sort(a, middle+1, end);
        merge(a, start, middle, end);
    }
}

/// The [start] index and [end] index (not length) is split by the [middle] index.
/// The sub lists are then merged into a.
fn merge(a: &mut Vec<i32>, start: usize, middle: usize, end: usize) {
    // todo: copy start - middle (slicing would just be a reference and we need to maintain order, so copy)
    let left = a[start..=middle].to_vec();
    let right = a[(middle+1)..=end].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while k <= end {
        if i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                a[k] = left[i];
                i += 1;
            } else {
                a[k] = right[j];
                j += 1;
            }
        } else if i < left.len() {
            a[k] = left[i];
            i += 1;
        } else if j < right.len() {
            a[k] = right[j];
            j += 1;
        }
        k += 1;
    }
}