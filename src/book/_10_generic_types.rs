#[cfg(test)]
mod tests {
    #[test]
    fn generics_work_as_expected_for_int() {
        let numbers = vec![34, 50, 25, 100, 65];

        let largest = find_largest(&numbers);

        assert_eq!(largest, 100);
    }

    #[test]
    fn generics_work_as_expected_for_char() {
        let chars = vec!['y', 'm', 'a', 'q'];

        let largest = find_largest(&chars);

        assert_eq!(largest, 'y');
    }

    #[test]
    fn generics_work_for_defined_type() {
        let fruits = vec![Fruit::Apple, Fruit::Banana, Fruit::Orange];

        let largest = find_largest(&fruits);

        assert_eq!(largest, Fruit::Banana);
    }

    fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        *largest
    }

    #[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
    enum Fruit {
        Apple = 1,
        Orange = 2,
        Banana = 3,
    }
}
