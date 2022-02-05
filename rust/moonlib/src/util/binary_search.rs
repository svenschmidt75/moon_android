/// Upper bound is defined as a function that finds the 1st element in
/// a slice where data[i] > to_find. Note: >, NOT >= !
/// In: data, slice with data. Must be sorted
/// to_find: Item to search for
pub(crate) fn upper_bound<T: std::cmp::PartialOrd>(data: &[T], to_find: &T) -> usize {
    let mut min_idx = 0;
    let mut max_idx = data.len();

    while min_idx < max_idx {
        let mid_idx = min_idx + (max_idx - min_idx) / 2;

        if *to_find >= data[mid_idx] {
            min_idx = mid_idx + 1;
        } else {
            max_idx = mid_idx;
        }
    }

    if min_idx < data.len() && data[min_idx] <= *to_find {
        min_idx += 1;
    }

    min_idx
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn upper_bound_less_than_first_test() {
        // Arrange
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let to_find = 0;

        // Act
        let idx = upper_bound(&data, &to_find);

        // Assert
        assert_eq!(0, idx)
    }

    #[test]
    fn upper_bound_grater_than_last_test() {
        // Arrange
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let to_find = 10;

        // Act
        let idx = upper_bound(&data, &to_find);

        // Assert
        assert_eq!(8, idx)
    }

    #[test]
    fn upper_bound_with_duplicates_test() {
        // Arrange
        let data = [10, 10, 10, 20, 20, 20, 30, 30];
        let to_find = 20;

        // Act
        let idx = upper_bound(&data, &to_find);

        // Assert
        assert_eq!(6, idx)
    }
}
