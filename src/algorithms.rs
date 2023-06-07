pub fn bubble_sort(arr: &mut [i32]) -> i32 {
    let mut iterations = 0;
    let len = arr.len();

    if len > 100_000 {
        return 0;
    }

    for i in 0..len {
        iterations += 1;

        for j in 0..len - i - 1 {
            iterations += 1;

            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    iterations
}

pub fn selection_sort(arr: &mut [i32]) -> i32 {
    let mut iterations = 0;

    let len = arr.len();

    for i in 0..len {
        iterations += 1;

        let mut min = i;

        for j in (i + 1)..len {
            iterations += 1;

            if arr[j] < arr[min] {
                min = j;
            }
        }

        arr.swap(i, min)
    }

    iterations
}

pub fn insertion_sort(arr: &mut [i32]) -> i32 {
    let mut iterations = 0;

    let len = arr.len();

    for i in 0..len {
        iterations += 1;

        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            iterations += 1;

            arr.swap(j, j - 1);
            j -= 1;
        }
    }

    iterations
}

pub fn merge_sort(arr: &mut [i32]) -> i32 {
    do_merge_sort(arr, &mut 0)
}

fn do_merge_sort(arr: &mut [i32], iterations: &mut i32) -> i32 {
    *iterations += 1;

    let len = arr.len();

    if len < 2 {
        return *iterations;
    }

    let mid = len / 2;
    do_merge_sort(&mut arr[..mid], iterations);
    do_merge_sort(&mut arr[mid..], iterations);

    let mut merged = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        *iterations += 1;

        if arr[i] <= arr[j] {
            merged.push(arr[i]);
            i += 1;
        } else {
            merged.push(arr[j]);
            j += 1;
        }
    }

    while i < mid {
        *iterations += 1;

        merged.push(arr[i]);
        i += 1;
    }

    while j < len {
        *iterations += 1;

        merged.push(arr[j]);
        j += 1;
    }

    arr.copy_from_slice(&merged[..]);

    *iterations
}

pub fn heap_sort(arr: &mut [i32]) -> i32 {
    let mut iterations = 0;

    build_max_heap(arr, &mut iterations);

    let len = arr.len();

    for i in (1..len).rev() {
        arr.swap(0, i);

        max_heapify(&mut arr[..i], 0, &mut iterations);
    }

    iterations
}

fn build_max_heap(arr: &mut [i32], iterations: &mut i32) {
    let len = arr.len();
    let start = len / 2;

    for i in (0..start).rev() {
        *iterations += 1;

        max_heapify(arr, i, iterations);
    }
}

fn max_heapify(arr: &mut [i32], mut i: usize, iterations: &mut i32) {
    let len = arr.len();

    loop {
        *iterations += 1;

        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < len && arr[left] > arr[largest] {
            largest = left;
        }

        if right < len && arr[right] > arr[largest] {
            largest = right;
        }

        if largest == i {
            break;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

pub fn quick_sort(arr: &mut [i32]) -> i32 {
    let mut iterations = 0;

    do_quick_sort(arr, &mut iterations);

    iterations
}
pub fn do_quick_sort(arr: &mut [i32], iterations: &mut i32) {
    *iterations += 1;

    let len = arr.len();

    if len < 2 {
        return;
    }

    let pivot_index = partition(arr, iterations);

    do_quick_sort(&mut arr[..pivot_index], iterations);
    do_quick_sort(&mut arr[pivot_index + 1..len], iterations);
}

fn partition(arr: &mut [i32], iterations: &mut i32) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);

    let mut i = 0;

    for j in 0..len - 1 {
        *iterations += 1;

        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);

    i
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng};

    const SIZE: i32 = 10;
    const SORTED_ARR: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    #[test]
    fn bubble_sort() {
        let mut arr = shuffled_arr();
        super::bubble_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn selection_sort() {
        let mut arr = shuffled_arr();
        super::selection_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn insertion_sort() {
        let mut arr = shuffled_arr();
        super::insertion_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn merge_sort() {
        let mut arr = shuffled_arr();
        super::merge_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn heap_sort() {
        let mut arr = shuffled_arr();
        super::heap_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    #[test]
    fn quick_sort() {
        let mut arr = shuffled_arr();
        super::quick_sort(&mut arr);

        assert_eq!(arr, SORTED_ARR);
    }

    fn shuffled_arr() -> Vec<i32> {
        let mut arr: Vec<_> = (0..SIZE).collect();

        arr.shuffle(&mut thread_rng());

        arr
    }
}
