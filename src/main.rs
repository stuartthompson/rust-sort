mod bubble_sort;
mod counting_sort;
mod radix_sort;
use bubble_sort::bubble_sort;
use counting_sort::counting_sort;
use radix_sort::radix_sort;

fn main() {
    // Counting sort
    let list = vec![4,5,1,7,6,5];
    let sorted = counting_sort(&list, 1);
    
    println!("Counting sort {:?} => {:?}", list, sorted);

    // Bubble sort
    let list = vec![5,1,7,3,4,3];
    let sorted = bubble_sort(&list);

    println!("  Bubble sort {:?} => {:?}", list, sorted);

    let list = vec![1938, 1921, 1985, 1964];
    let sorted = radix_sort(&list);

    println!("   Radix sort {:?} => {:?}", list, sorted);
}
