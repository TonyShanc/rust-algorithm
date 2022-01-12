pub fn heap_sort<T: Ord>(arr: &mut [T]) {

    build_heap(arr);

    for i in (0..arr.len()).rev() {
        arr.swap(i, 0);
        sink(&mut arr[..i], 0)
    }
}

fn build_heap<T: Ord>(arr: &mut [T]) {
    for i in (0..(arr.len() - 2) / 2 + 1).rev() {
        sink(arr, i)
    }
}

fn sink<T: Ord>(arr: &mut [T], i: usize) {

    let mut cur = i;
    loop {
        let mut tmp = cur;
        let left = 2 * cur + 1;
        let right = 2 * cur + 2;

        if left < arr.len() && arr[tmp] < arr[left] {
            tmp = left;
        }

        if right < arr.len() && arr[tmp] < arr[right] {
            tmp = right;
        }

        if cur == tmp {
            break
        }

        arr.swap(cur, tmp);

        cur = tmp;
    }
}

fn get_slice_len(arr: &mut [i32]) -> usize{
    return arr.len();
}

mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut arr = vec![6,5,4,3,2,1];
        heap_sort(&mut arr);
        println!("{:?}", arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] < arr[i + 1])
        }
    }

    #[test]
    fn ascending() {
        let mut arr = vec![1,2,3,4,5,6];
        heap_sort(&mut arr);
        println!("{:?}", arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] < arr[i + 1])
        }
    }

    #[test]
    fn test_slice_len() {
        let mut arr = vec![3,2,1];
        println!("!!!{}", arr.len());
        println!("{}", get_slice_len(&mut arr[0..1]))
    }
}