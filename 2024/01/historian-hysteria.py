list_1: list[int] = []
list_2: list[int] = []

with open('input.txt', 'r') as file:
    for line in file:
        [input_1, input_2] = line.strip().split()
        list_1.append(int(input_1))
        list_2.append(int(input_2))

list_1.sort()
list_2.sort()

sum = sum([abs(list_1[i] - list_2[i]) for i in range(len(list_1))])
print('The answer is', sum)
