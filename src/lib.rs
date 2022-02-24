pub fn shape_2D<T>(v: Vec<T>, column_count: u8) -> Vec<Vec<T>> {
    let mut shaped = vec![];
    let mut row = vec![];
    for element in v {
        row.push(element);
        if row.len() == column_count as usize {
            shaped.push(row);
            row = vec![];
        }
    }
    shaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(shape_2D(v, 3), expected);
    }
}
