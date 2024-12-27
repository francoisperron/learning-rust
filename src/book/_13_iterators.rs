// https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[cfg(test)]
mod tests {
    #[test]
    fn all_works() {
        let all_even = [2, 4, 6].iter().all(|i| i % 2 == 0);
        assert_eq!(all_even, true);
    }

    #[test]
    fn any_works() {
        let any_even = [1, 3, 6].iter().any(|i| i % 2 == 0);
        assert_eq!(any_even, true);
    }

    #[test]
    fn count_works() {
        let count = [1, 2, 3, 4].iter().count();
        assert_eq!(count, 4);
    }

    #[test]
    fn cycle_works() {
        let mut numbers = [1, 2].into_iter().cycle();

        assert_eq!(numbers.next().unwrap(), 1);
        assert_eq!(numbers.next().unwrap(), 2);
        assert_eq!(numbers.next().unwrap(), 1);
        assert_eq!(numbers.next().unwrap(), 2);
    }

    #[test]
    fn sum_works() {
        let sum: i32 = [1, 2, 3].iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn min_max_works() {
        let min_number = [1, 2, 3].into_iter().min().unwrap();
        assert_eq!(min_number, 1);

        let max_number = [1, 2, 3].into_iter().max().unwrap();
        assert_eq!(max_number, 3);

        let empty: [i32; 0] = [];
        let no_max_number = empty.iter().max();
        assert_eq!(no_max_number, None);
    }

    #[test]
    fn filter_works() {
        let even_numbers: Vec<_> = [1, 2, 3].into_iter().filter(|i| i % 2 == 0).collect();

        assert_eq!(even_numbers, vec![2])
    }

    #[test]
    fn filter_map_works() {
        let even_numbers: Vec<_> = [1, 2, 3]
            .into_iter()
            // .filter_map(|&i| if i % 2 == 0 { Some(i) } else { None })
            .filter_map(|i| match i % 2 == 0 {
                true => Some(i),
                false => None,
            })
            .collect();
        assert_eq!(even_numbers, vec![2])
    }

    #[test]
    fn map_works() {
        let double_numbers: Vec<_> = [1, 2, 3].iter().map(|i| i * 2).collect();

        assert_eq!(double_numbers, vec![2, 4, 6])
    }

    #[test]
    fn reduce_works() {
        let sum_range = (1..10).reduce(|sum, i| sum + i);
        assert_eq!(sum_range, Some(45));

        let sum_fold_array = [1, 2, 3].iter().fold(0, |sum, i| sum + i);
        assert_eq!(sum_fold_array, 6);

        let sum_reduce_array = [1, 2, 3].into_iter().reduce(|sum, i| sum + i).unwrap();
        assert_eq!(sum_reduce_array, 6);
    }

    #[test]
    fn inspect_seems_helpful() {
        let sum = [1, 4, 2, 3]
            .into_iter()
            .inspect(|x| println!("about to filter: {x}"))
            .filter(|x| x % 2 == 0)
            .inspect(|x| println!("made it through filter: {x}"))
            .fold(0, |sum, i| sum + i);

        assert_eq!(sum, 6);
    }

    #[test]
    fn first_and_last_works() {
        let data = [1, 2, 3];

        assert_eq!(data.into_iter().next().unwrap(), 1);
        assert_eq!(data.into_iter().last().unwrap(), 3);
    }

    #[test]
    fn partition_groups_by() {
        let (even, odd): (Vec<_>, Vec<_>) = [1, 2, 3].into_iter().partition(|i| i % 2 == 0);

        assert_eq!(even, vec![2]);
        assert_eq!(odd, vec![1, 3]);
    }

    #[test]
    fn reverse_works() {
        let rev: Vec<_> = [1, 2, 3].into_iter().rev().collect();
        assert_eq!(rev, vec![3, 2, 1]);
    }

    #[test]
    fn slice_works() {
        let numbers = [1, 2, 3, 4, 5];

        let first_two_numbers = &numbers[..2];
        assert_eq!(first_two_numbers, [1, 2]);

        let last_two_numbers = &numbers[3..];
        assert_eq!(last_two_numbers, [4, 5]);

        let middle_three_numbers = &numbers[1..4];
        assert_eq!(middle_three_numbers, [2, 3, 4]);
    }

    #[test]
    fn sort_works() {
        let mut numbers = [3, 1, 4, 5, 2];

        numbers.sort();

        assert_eq!(numbers, [1, 2, 3, 4, 5])
    }
}
