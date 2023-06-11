mod algorithm_result;
mod algorithms;

use algorithm_result::AlgorithmResult;
use algorithms::{
    bubble_sort, heap_sort, improved_bubble_sort, improved_quick_sort, improved_selection_sort,
    insertion_sort, merge_sort, quick_sort, selection_sort,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::File,
    io::{LineWriter, Write},
    time::Instant,
};

// Define o tamanho dos arrays
const SIZES: [i32; 6] = [1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000];

// Declaração de tipos para ficar menos verboso
type SorterFn = fn(i32) -> Vec<i32>;
type AlgorithmFn = fn(&mut [i32]);
type Summary = Vec<AlgorithmResult>;

fn main() {
    // Lista de funções de inicialização dos arrays
    let sorters = [
        ("Ordenado", sorted_arr as SorterFn),
        ("Invertido", reversed_arr),
        ("Quase ordenado", almost_sorted_arr),
        ("Aleatório", shuffled_arr),
    ];

    // Lista dos algoritmos
    let algorithms = [
        ("Bubble sort", bubble_sort as AlgorithmFn),
        ("Bubble sort melhorado", improved_bubble_sort),
        ("Selection sort", selection_sort),
        ("Selection sort melhorado", improved_selection_sort),
        ("Insertion sort", insertion_sort),
        ("Merge sort", merge_sort as AlgorithmFn),
        ("Heap sort", heap_sort),
        ("Quick sort", quick_sort as AlgorithmFn),
        ("Quick sort melhorado", improved_quick_sort),
    ];

    // Variável pra guardar todos os resultados
    let mut results = Vec::new();

    for size in SIZES {
        println!("----------------------------");
        for (sort_type, sorter) in sorters {
            // Inicializa o array, para iteração será um tipo de inicialização diferente (Ordenado,
            // Invertido, Quase ordenado e aletaório)
            let arr = sorter(size);

            for (algorithm_name, algorithm) in algorithms {
                println!("{size} - {sort_type} - {algorithm_name}");

                let now = Instant::now();

                // Chama a função que realizará o sort efetivamente, aqui é feito um clone para a
                // variável `arr` não ser alterada e poder ser usada nas próximas iterações
                algorithm(&mut arr.clone());

                // Armazena o resultado
                results.push(AlgorithmResult {
                    algorithm: algorithm_name.to_owned(),
                    arr_size: size,
                    sort_type: sort_type.to_owned(),
                    duration: now.elapsed(),
                    duration_ms: (now.elapsed().as_nanos()) as f64 / 1000000.0,
                })
            }
        }
    }

    // Salva os resultados num CSV
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
            "{},{},{},{:.2?},{}\n",
            algorithm_result.arr_size,
            algorithm_result.algorithm,
            algorithm_result.sort_type,
            algorithm_result.duration,
            algorithm_result.duration_ms,
        );

        file.write_all(row.as_bytes()).unwrap();
    }

    file.flush().unwrap();
}

// Cria um array ordenado
fn sorted_arr(size: i32) -> Vec<i32> {
    (0..size).collect()
}

// Cria um array invertido
fn reversed_arr(size: i32) -> Vec<i32> {
    (0..size).rev().collect()
}

// Cria um array quase ordenado (1/3 aletaório)
fn almost_sorted_arr(size: i32) -> Vec<i32> {
    let mut arr: Vec<_> = (0..size).collect();

    let end = arr.len() / 3;
    let range = &mut arr[..end];

    range.shuffle(&mut rand::thread_rng());

    arr
}

// Cria um array aleatório
fn shuffled_arr(size: i32) -> Vec<i32> {
    let mut arr: Vec<_> = (0..size).collect();

    arr.shuffle(&mut thread_rng());

    arr
}
