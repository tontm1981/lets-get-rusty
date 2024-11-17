pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::selection_sort(arr);
}

mod sorting {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let length: usize = arr.len();
        for i in 0..length {
            let mut min_index: usize = i;
            for j in i+1..length {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    #[allow(dead_code)]
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let length: usize = arr.len();
        for i in 0..length {
            for j in 0..length-i-1 {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            bubble_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }

        #[test]
        fn test_selection_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            selection_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }
    }
}