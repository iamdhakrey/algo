def generateList(count_number) -> list[int]:
    numbers_list: list[int] = []
    for i in range(count_number):
        numbers_list.append(i)

    return numbers_list

class BinarySearch:
    def __init__(self,numbers_list: list[int], search_number: int):
        self.numbers_list: list[int] = numbers_list
        self.search_number = search_number

    def search(self):
        start:int  = 1
        end:int  = len(self.numbers_list)
        found: bool = False
        while start <= end:
            if start > end:
                start, end = end, start

            middle: int  = start + (start - end)% 2
            print(middle)
            if middle > len(self.numbers_list) or middle < 0:
                print("not found")
                # exit()
                break

            if self.numbers_list[middle] > self.search_number:
                end = middle -1
            if self.numbers_list[middle] < self.search_number:
                start  = middle +1
            if self.search_number == self.numbers_list[middle]:
                found = True
                print("number found")
                break

        if not found:
            print("not found")
        else:
            print("found")

numbers_list = generateList(100)
numbers_list = [1,2,3,4,6,7,8,9]
b = BinarySearch(numbers_list, 5)
b.search()
