pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() == 0 {
        return
    }
    for i in 1..arr.len() {
        let mut j = i;
        loop {
            if arr[j] < arr[j - 1]{
                arr.swap(j, j - 1);
                j  = j - 1;
            }
            if j == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending(){

        let mut vec1 = vec![6,5,4,3,2,1];
        insertion_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] < vec1[i + 1])
        }
    }

    #[test]
    fn ascending() {
        let mut vec1 = vec![1,2,3,4,5,6];
        insertion_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] < vec1[i + 1])
        }
    }
}