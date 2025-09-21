use std::collections::HashMap;

pub fn calculate_stats(mut numbers: Vec<i32>) -> (f64, i32) {
    let len  = numbers.len();

    match len {
        0 => (0.0, 0),
        1 => (numbers[0] as f64, numbers[0]),
        _ => {
            // median calculation
            numbers.sort();
            let median = if len % 2 == 0 {
                (numbers[len/2 -1] as f64 + numbers[len/2] as f64) / 2.0
            } else {
                numbers[len/2] as f64
            };

            //mode calculation
            let mut mode = 0;
            let mut frequency_map: HashMap<i32, u32> = HashMap::new();
            for &num in &numbers { 
                frequency_map.entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            let mut max_count = 0;
            for (&num, &count) in &frequency_map{
                if count > max_count {
                    mode = num;
                    max_count = count;
                }
            }
            (median, mode)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_odd_length_any_mode() {
        let numbers = vec![1, 2, 3, 4, 5];
        let (median, mode) = calculate_stats(numbers);
        assert_eq!(median, 3.0);
        assert!(mode >= 1 && mode <= 5); // mode can be any value in this case
    }

    #[test]
    fn test_2_odd_length_single_mode() {
        let numbers = vec![1, 2, 2, 3, 4];
        let (median, mode) = calculate_stats(numbers);
        assert_eq!(median, 2.0);
        assert_eq!(mode, 2);
    }

    #[test]
    fn test_3_even_length_any_mode() {
        let numbers = vec![1, 1, 2, 2, 3, 3];
        let (median, mode) = calculate_stats(numbers);
        assert_eq!(median, 2.0);
        assert!(mode == 1 || mode == 2 || mode == 3); // mode can be any of 1, 2, or 3
    }

    #[test]
    fn test_4_single_element() {
        let numbers = vec![5];
        let (median, mode) = calculate_stats(numbers);
        assert_eq!(median, 5.0);
        assert_eq!(mode, 5);
    }
}
