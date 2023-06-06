import csv
import algorithms
import random

# SIZES = [ 1_000, 10_000, 50_000, 100_000, 500_000, 1_000_000]
SIZES = [10]
ALGORITHMS = [
    algorithms.bubble_sort,
    algorithms.improved_bubble_sort,
    algorithms.selection_sort,
    algorithms.insertion_sort,
]


def main():
    results = []

    for size in SIZES:
        print("-------------------------")
        print(f"Tamanho do array: {size}")
        arr = [*range(size)]

        random.shuffle(arr)

        for algorithm in ALGORITHMS:
            result = algorithm(arr[:])

            results.append(result)

    print("\nSalvando CSV...")
    save_to_csv(results)


def save_to_csv(results):
    with open('results.csv', 'w', newline='') as file:
        writer = csv.writer(file)
        field = ["Tamanho", "Nome", "Duração (ms)", "Iterações"]

        writer.writerow(field)

        for result in results:
            writer.writerow([result.size,
                             result.algorithm,
                             result.duration / 1_000_000,
                             result.iterations])


if __name__ == "__main__":
    main()
