import time


def bubble_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)

    for _ in range(n - 1):
        iterations += 1

        for j in range(0, n - 1):
            iterations += 1

            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]

    return (time.time_ns() - time_ns, iterations)


def improved_bubble_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)
    swapped = False

    for i in range(n - 1):
        iterations += 1

        for j in range(0, n - i - 1):
            iterations += 1

            if arr[j] > arr[j + 1]:
                swapped = True
                arr[j], arr[j + 1] = arr[j + 1], arr[j]

        if not swapped:
            print(arr)
            return (time.time_ns() - time_ns, iterations)

    return (time.time_ns() - time_ns, iterations)
