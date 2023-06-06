import random
import unittest
import algorithms

SIZE = 10
shuffled_arr = [*range(SIZE)]
sorted_arr = [*range(SIZE)]


class TestAlgorithms(unittest.TestCase):
    def setUp(self):
        random.shuffle(shuffled_arr)

    def test_bubble_sort(self):
        algorithms.bubble_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_improved_bubble_sort(self):
        algorithms.bubble_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_selection_sort(self):
        algorithms.selection_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_insertion_sort(self):
        algorithms.insertion_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_merge_sort(self):
        algorithms.merge_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_heap_sort(self):
        algorithms.heap_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)

    def test_quick_sort(self):
        algorithms.quick_sort(shuffled_arr)
        self.assertEqual(shuffled_arr, sorted_arr)


if __name__ == '__main__':
    unittest.main()
