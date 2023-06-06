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


def selection_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)

    for i in range(n):
        iterations += 1

        min = i

        for j in range(i + 1, n):
            iterations += 1

            if arr[j] < arr[min]:
                min = j

        arr[i], arr[min] = arr[min], arr[i]

    return (time.time_ns() - time_ns, iterations)


def insertion_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)

    for i in range(n):
        iterations += 1

        j = i

        while j > 0 and arr[j - 1] > arr[j]:
            iterations += 1

            arr[j], arr[j - 1] = arr[j - 1], arr[j]
            j -= 1

    return (time.time_ns() - time_ns, iterations)


