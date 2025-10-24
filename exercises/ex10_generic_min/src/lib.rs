pub fn min_value<T: PartialOrd>(values: &[T]) -> Option<&T> {
    if values.len() == 0 {
        return None;
    }

    let mut min = &values[0];

    for item in values {
        if item < min {
            min = item;
        }
    }

    Some(min)
}