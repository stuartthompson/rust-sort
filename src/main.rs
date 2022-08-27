mod bubble_sort;
mod counting_sort;
mod radix_sort;
use bubble_sort::bubble_sort;
use counting_sort::counting_sort;
use radix_sort::radix_sort;

fn main() {
    // Counting sort
    let list = vec![4,5,1,7,6,5];
    println!("Counting sort {:?} => {:?}", list, counting_sort(&list, 1));

    // Bubble sort
    let list = vec![45, 31, 64, 72, 46, 5, 1];
    println!("  Bubble sort {:?} => {:?}", list, bubble_sort(&list));

    // Radix sort
    let list = vec![1938, 1921, 1985, 1964];
    println!("   Radix sort1 {:?} => {:?}", list,  radix_sort(&list));
    let list = vec![1934, 651, 77712, 19, 173, 31];
    println!("   Radix sort2 {:?} => {:?}", list, radix_sort(&list));
}
