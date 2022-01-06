use std::cmp::Ordering;

pub fn binary_search<T: Ord>(items: &[T], k: &T) -> Option<usize>{
    let mut left = 0;
    let mut right = items.len();

    while left <= right {
        let mid = left + (right - left) / 2;

        match k.cmp(&items[mid]) {
            Ordering::Less => right = mid - 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }

    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_ints() {
        let index = binary_search(&vec![1, 2, 3, 4], &4);
        assert_eq!(index, Some(3));

        let index = binary_search(&vec![1, 2, 3, 4], &3);
        assert_eq!(index, Some(2));

        let index = binary_search(&vec![1, 2, 3, 4], &2);
        assert_eq!(index, Some(1));

        let index = binary_search(&vec![1, 2, 3, 4], &1);
        assert_eq!(index, Some(0));
    }
}
