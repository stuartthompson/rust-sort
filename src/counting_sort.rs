/// Returns a sorted list using the counting sort algorithm.
/// 
/// The algorithm will sort the entries according to a specific digit of the
///  values in the list, i.e., 1973, 1936 sorted by digit 1 (rightmost) would
///  be 1973, 1936.
/// 
/// # Arguments
/// 
/// * `list` - The list to sort.
/// * `digit` - The digit to sort by.
pub fn counting_sort(
    list: &Vec<u32>,
    digit: u8
) -> Vec<u32> {
    // Declare a working array
    let mut working = vec![0; 10];
    let mut result = vec![0; list.len()];

    let base: u32 = 10;
    let get_digit = 
        |n: &u32, digit: u8| (n / (base.pow((digit - 1)as u32) as u32)) % 10;

    // Count how many times each element appears in the list
    // Store the count of elements in the index identified by that element.
    // i.e., if there are two occurrences of 4 in the list, then the 4th
    //  element in the counts array should be 2.
    for number in list {
        let digit_value = get_digit(number, digit);
        working[digit_value as usize]+=1;
    }

    // Accumulate a running total in the list
    for i in 0..working.len() {
        if i == 0 { continue; }
        working[i] += working[i - 1];
    }

    // Rotate the array to the right
    working.rotate_right(1);
    working[0] = 0;

    // Walk the original list and place the elements into an output list
    for i in 0..list.len() {
        // Get the element to place
        let element = list[i];
        // Look up the index at which to place this element
        let ix = working[get_digit(&element, digit) as usize];
        // Place the element in the output array
        result[ix] = element;
        // Increment the index for this element (i.e., where does next 4 go?)
        working[get_digit(&element, digit) as usize]+=1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_single_digit_list() {
        let list = vec![3,5,1,1,2];
        let sorted = counting_sort(&list, 1);
        assert_eq!(sorted, [1,1,2,3,5]);
    }

    #[test]
    fn sort_two_digit_list() {
        let list = vec![45, 61, 32, 97, 51];
        let sorted = counting_sort(&list, 1);
        assert_eq!(sorted, [61, 51, 32, 45, 97]);
    }

    #[test]
    fn sort_three_digit_list() {
        let list = vec![143, 515, 612];
        let sorted = counting_sort(&list, 2);
        assert_eq!(sorted, [515, 612, 143]);
    }

    #[test]
    fn sort_different_size_list() {
        let list = vec![51, 7, 194, 63];
        let sorted = counting_sort(&list, 2);
        assert_eq!(sorted, [7, 51, 63, 194]);
    }
}