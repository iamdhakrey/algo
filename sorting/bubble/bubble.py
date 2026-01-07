"""
Bubble Sort

How it works:

Go through the array, one value at a time.
For each value, compare the value with the next value.
If the value is higher than the next one, swap the values so that the highest value comes last.
Go through the array as many times as there are values in the array.

Time Complexity: O(n2)
"""

array = [64, 34, 25, 12, 22, 11, 90, 5]

n = len(array)
for i in range(n-1):
    for j in range(n-i-1):
        if array[j] > array[j+1]:
            array[j], array[j+1] = array[j+1],array[j]

print("sorted array:", array)
