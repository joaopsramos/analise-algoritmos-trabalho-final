mod algorithm_result;
mod algorithms;

use std::{
    fs::File,
    io::{LineWriter, Write},
    time::Instant,
};

use algorithms::{
    bubble_sort, heap_sort, improved_bubble_sort, improved_selection_sort, insertion_sort,
    merge_sort, quick_sort, selection_sort,
};
use rand::{seq::SliceRandom, thread_rng};

use algorithm_result::AlgorithmResult;

const SIZES: [i32; 6] = [1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000];

type SorterFn = fn(i32) -> Vec<i32>;
type AlgorithmFn = fn(&mut [i32]) -> i64;
type Summary = Vec<AlgorithmResult>;

fn main() {
    let sorters = [
        ("Ordenado", sorted_arr as SorterFn),
        ("Invertido", reversed_arr),
        ("Quase ordenado", almost_sorted_arr),
        ("Aleatório", shuffled_arr),
    ];
    let algorithms = [
        ("Bubble sort", bubble_sort as AlgorithmFn),
        ("Bubble sort melhorado", improved_bubble_sort),
        ("Selection sort", selection_sort),
        ("Selection sort melhorado", improved_selection_sort),
        ("Insertion sort", insertion_sort),
        ("Merge sort", merge_sort as AlgorithmFn),
        ("Heap sort", heap_sort),
        ("Quick sort", quick_sort),
    ];

    let mut results = Vec::new();

    for size in SIZES {
        println!("----------------------------");
        for (sort_type, sorter) in sorters {
            let arr = sorter(size);

            for (algorithm_name, algorithm) in algorithms {
                println!("{size} - {sort_type} - {algorithm_name}");

                let now = Instant::now();

                let iterations = algorithm(&mut arr.clone());

                results.push(AlgorithmResult {
                    algorithm: algorithm_name.to_owned(),
                    arr_size: size,
                    sort_type: sort_type.to_owned(),
                    duration: now.elapsed(),
                    duration_ms: (now.elapsed().as_nanos()) as f64 / 1000000.0,
                    iterations,
                })
            }
        }
    }

    save_to_csv(results);
}

fn save_to_csv(results: Summary) {
    let file = File::create("./results.csv").unwrap();
    let mut file = LineWriter::new(file);
    file.write_all(
        "Tamanho,Algoritmo,Tipo de ordenação,Duração,Duração (ms),Iterações\n".as_bytes(),
    )
    .unwrap();

    for algorithm_result in results {
        let row = format!(
            "{},{},{},{:.2?},{},{}\n",
            algorithm_result.arr_size,
            algorithm_result.algorithm,
            algorithm_result.sort_type,
            algorithm_result.duration,
            algorithm_result.duration_ms,
            algorithm_result.iterations
        );

        file.write_all(row.as_bytes()).unwrap();
    }

    file.flush().unwrap();
}

fn sorted_arr(size: i32) -> Vec<i32> {
    (0..size).collect()
}

fn reversed_arr(size: i32) -> Vec<i32> {
    (0..size).rev().collect()
}

fn almost_sorted_arr(size: i32) -> Vec<i32> {
    let mut arr: Vec<_> = (0..size).collect();

    let end = arr.len() / 3;
    let range = &mut arr[..end];

    range.shuffle(&mut rand::thread_rng());

    arr
}

fn shuffled_arr(size: i32) -> Vec<i32> {
    let mut arr: Vec<_> = (0..size).collect();

    arr.shuffle(&mut thread_rng());

    arr
}
