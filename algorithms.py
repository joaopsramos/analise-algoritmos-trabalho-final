import time
from result import Result


def bubble_sort(arr):
    name = "Bubble sort"
    print(name)

    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)

    for _ in range(n - 1):
        iterations += 1

        for j in range(0, n - 1):
            iterations += 1

            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]

    return Result(n, name, time.time_ns() - time_ns, iterations)


def improved_bubble_sort(arr):
    name = "Bubble sort melhorado"
    print(name)

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

    return Result(n, name, time.time_ns() - time_ns, iterations)


def selection_sort(arr):
    name = "Selection sort"
    print(name)

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

    return Result(n, name, time.time_ns() - time_ns, iterations)


def insertion_sort(arr):
    name = "Insertion sort"
    print(name)

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

    return Result(n, name, time.time_ns() - time_ns, iterations)

def quick_sort(arr):
    name = "Quick sort"
    print(name)

    time_ns = time.time_ns()
    iterations = 0

    n = len(arr)

    iterations = do_quick_sort(arr, 0, n - 1, iterations)

    return Result(n, name, time.time_ns() - time_ns, iterations)

def do_quick_sort(arr, low, high, iterations):
    iterations += 1

    if low < high:
        pivot = partition(arr, low, high)
     
        iterations = do_quick_sort(arr, low, pivot - 1, iterations)
        iterations = do_quick_sort(arr, pivot + 1, high, iterations)

    return iterations

def partition(arr, low, high):
    pivot = arr[high]
    swap_marker = low - 1

    for i in range(low, high):
        if arr[i] <= pivot:
            swap_marker += 1
            
            arr[i], arr[swap_marker] = arr[swap_marker], arr[i]

    arr[swap_marker + 1], arr[high] = arr[high], arr[swap_marker + 1]

    return swap_marker + 1


