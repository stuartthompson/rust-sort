pub fn bubble_sort(list: &Vec<u8>) -> Vec<u8> {
    let mut sorted = list.clone();
    // Loop through the array, swapping adjacent entries if needed
    loop {
        let mut swapped = 0;
        for i in 0..sorted.len() - 1 {
            if sorted[i] > sorted[i + 1] {
                let temp = sorted[i + 1];
                sorted[i + 1] = sorted[i];
                sorted[i] = temp;
                swapped+=1;
            }
        }
        // Stop looping when no swaps were made
        if swapped == 0 {
            break;
        }
    }
    return sorted;
}