/// Returns a sorted list using the counting sort algorithm.
/// 
/// # Arguments
/// 
/// * `list` - The list to sort.
pub fn counting_sort(list: &Vec<u8>) -> Vec<u8> {
    // Declare a working vec to use for counting and placement indexes
    // Length is the range of numbers in the list (one slot per value)
    let mut range = 0;
    for i in 0..list.len() {
        if list[i] > range { range = list[i]; }
    }
    let mut working = vec![0; range as usize + 1];

    // Count how many times each element appears in the list
    // Store the count of elements in the index identified by that element.
    // i.e., if there are two occurrences of 4 in the list, then the 4th
    //  element in the counts array should be 2.
    for number in list {
        working[*number as usize]+=1;
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
    let mut result = vec![0; list.len()];
    for i in 0..list.len() {
        // Get the element to place
        let element = list[i];
        // Look up the index at which to place this element
        let ix = working[element as usize];
        // Place the element in the output array
        result[ix] = element;
        // Increment the index for this element (i.e., where does next 4 go?)
        working[element as usize]+=1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_is_sorted() {
        let list = vec![3,5,1,1,2];

        let sorted_list = counting_sort(&list);

        assert_eq!(sorted_list, [1,1,2,3,5]);
    }
}