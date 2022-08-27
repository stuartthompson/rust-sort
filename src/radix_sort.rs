use crate::counting_sort;

pub fn radix_sort(list: &Vec<u32>) -> Vec<u32> {
    // Find the largest number in the list
    let mut max = 0;
    for number in list {
        if number > &max { 
            max = *number;
        }
    }

    // Find how many digits are in the largest number
    let mut digits = 0;
    let mut comparison = 1;
    let count = loop {
        if max > comparison {
            comparison = comparison * 10;
            digits += 1;
        }
        else {
            break digits;
        }
    };

    let mut result = list.clone();
    // Iteratively sort the list using a counting sort digit by digit RTL
    for i in 1..count+1 {
        // Pass the latest result (sorted list) in digit by digit
        result = counting_sort(&result, i); 
    }

    return result;
}