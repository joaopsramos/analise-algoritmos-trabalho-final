import csv
import algorithms
import random
from result import Result

SIZES = [1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000]
SIZES = [10]
ALGORITHMS = [
    ("Bubble sort", algorithms.bubble_sort),
    ("Bubble sort melhorado", algorithms.improved_bubble_sort),
    ("Selection sort", algorithms.selection_sort),
    ("Insertion sort", algorithms.insertion_sort),
    ("Merge sort", algorithms.merge_sort),
    ("Heap sort", algorithms.heap_sort),
    ("Quick sort", algorithms.quick_sort),
]


def sorted_arr(size):
    return [*range(size)]


def reversed_arr(size):
    return [*range(size - 1, -1, -1)]


def almost_sorted_arr(size):
    arr = [*range(size)]
    third = size // 3

    to_shuffle = arr[:third]
    random.shuffle(to_shuffle)

    return to_shuffle + arr[third:]


def shuffled_arr(size):
    arr = [*range(size)]
    random.shuffle(arr)

    return arr


SORT_TYPES = [
    ("Ordenado", sorted_arr),
    ("Invertido", reversed_arr),
    ("Quase ordenado", almost_sorted_arr),
    ("Desordenado", shuffled_arr)
]


def main():
    results = []

    for size in SIZES:
        print("-------------------------")

        for (sort_type, builder) in SORT_TYPES:
            arr = builder(size)

            for (name, algorithm) in ALGORITHMS:
                print(f"{size} - {sort_type} - {name}")

                time, iterations = algorithm(arr[:])

                results.append(Result(size, name, sort_type, time, iterations))

    print("\nSalvando CSV...")
    save_to_csv(results)


def save_to_csv(results):
    with open('results.csv', 'w', newline='') as file:
        writer = csv.writer(file)
        field = [
            "Tamanho",
            "Nome",
            "Tipo de ordenação",
            "Duração (ms)",
            "Iterações"]

        writer.writerow(field)

        for result in results:
            writer.writerow([result.size,
                             result.algorithm,
                             result.sort_type,
                             result.duration / 1_000_000,
                             result.iterations])


if __name__ == "__main__":
    main()
