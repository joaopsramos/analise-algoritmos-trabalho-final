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

    return time.time_ns() - time_ns, iterations


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
            return (time.time_ns() - time_ns, iterations)

    return time.time_ns() - time_ns, iterations


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

    return time.time_ns() - time_ns, iterations


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

    return time.time_ns() - time_ns, iterations


def merge_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    arr[:], iterations = do_merge_sort(arr, iterations)

    return time.time_ns() - time_ns, iterations


def do_merge_sort(arr, iterations):
    iterations += 1

    if len(arr) <= 1:
        return arr, iterations

    mid = len(arr) // 2
    left_half = arr[:mid]
    right_half = arr[mid:]

    left_half, iterations = do_merge_sort(left_half, iterations)
    right_half, iterations = do_merge_sort(right_half, iterations)

    return merge(left_half, right_half, iterations)


def merge(left, right, iterations):
    merged = []
    left_index = 0
    right_index = 0

    while left_index < len(left) and right_index < len(right):
        iterations += 1

        if left[left_index] < right[right_index]:
            merged.append(left[left_index])
            left_index += 1
        else:
            merged.append(right[right_index])
            right_index += 1

    merged += left[left_index:]
    merged += right[right_index:]

    return merged, iterations


def heap_sort(arr):
    time_ns = time.time_ns()
    iterations = do_heap_sort(arr)

    return time.time_ns() - time_ns, iterations


def do_heap_sort(arr):
    iterations = 0
    n = len(arr)

    for i in range(n // 2 - 1, -1, -1):
        iterations += 1

        iterations = heapify(arr, n, i, iterations)

    for i in range(n - 1, 0, -1):
        iterations += 1

        arr[i], arr[0] = arr[0], arr[i]
        iterations = heapify(arr, i, 0, iterations)

    return iterations


def heapify(arr, n, i, iterations):
    iterations += 1

    largest = i
    left = 2 * i + 1
    right = 2 * i + 2

    if left < n and arr[left] > arr[largest]:
        largest = left

    if right < n and arr[right] > arr[largest]:
        largest = right

    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]

        heapify(arr, n, largest, iterations)

    return iterations


def quick_sort(arr):
    time_ns = time.time_ns()
    iterations = 0

    stack = [(0, len(arr) - 1)]

    while stack:
        iterations += 1

        start, end = stack.pop()
        if start >= end:
            continue
        
        pivot = arr[start]
        left = start + 1
        right = end

        while True:
            iterations += 1

            while left <= right and arr[left] <= pivot:
                iterations += 1
                left += 1
            while left <= right and arr[right] >= pivot:
                iterations += 1
                right -= 1
            
            if left > right:
                break
            
            arr[left], arr[right] = arr[right], arr[left]
        
        arr[start], arr[right] = arr[right], arr[start]
        
        stack.append((start, right - 1))
        stack.append((right + 1, end))

    return time.time_ns() - time_ns, iterations

