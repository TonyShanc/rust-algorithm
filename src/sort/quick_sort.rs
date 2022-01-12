fn partition<T: PartialOrd>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        }else {
            arr.swap(i as usize, j as usize)
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

fn partition2<T: PartialOrd>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high  as usize;
    let mut j = low as usize;
    for i  in low as usize ..high as usize {
        if arr[i] < arr[pivot] {
            arr.swap(i, j);
            j += 1;
        }

    }
    arr.swap(j, pivot);
    j as isize
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn _quick_sort2<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition2(arr, low, high);
        _quick_sort2(arr, low, p - 1);
        _quick_sort2(arr, p + 1, high);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

pub fn quick_sort2<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort2(arr, 0, (len - 1) as isize);
}

mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut vec1 = vec![6,5,4,3,2,1];
        quick_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] < vec1[i + 1])
        }
    }

    #[test]
    fn ascending() {
        let mut arr = vec![1,2,3,4,5,6];
        quick_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] < arr[i + 1])
        }
    }

    #[test]
    fn descending2() {
        let mut vec1 = vec![6,5,4,3,2,1];
        quick_sort2(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] < vec1[i + 1])
        }
    }

    #[test]
    fn ascending2() {
        let mut arr = vec![1,2,3,4,5,6];
        quick_sort2(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] < arr[i + 1])
        }
    }
}