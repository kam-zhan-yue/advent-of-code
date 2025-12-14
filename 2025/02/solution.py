real_input = open('../inputs/02.txt').read()

def get_digits(num: int):
    digits = 1
    val = num // 10
    while val > 0:
        val = val // 10
        digits += 1
    return digits

def part_one(input: str):
    total = 0
    pairs = input.split(',')
    for pair in pairs:
        nums = pair.split('-')
        left = int(nums[0])
        right = int(nums[1])
        for i in range(right - left + 1):
            num = left + i
            digits = get_digits(num)
            if digits % 2 != 0:
                continue
            if num % (pow(10, digits // 2) + 1) == 0:
                total += num

    print("Part One is:", total)


def is_repeat(word: str) -> bool:
    for i in range(1, len(word) // 2 + 1):
        sub = word[0:i]
        repeat = len(word) // i
        if sub * repeat == word:
            return True
    return False


def part_two(input: str):
    total = 0
    pairs = input.split(',')
    for pair in pairs:
        nums = pair.split('-')
        left = int(nums[0])
        right = int(nums[1])
        for i in range(right - left + 1):
            num = left + i
            if is_repeat(str(num)):
                total += num

    print("Part Two is:", total)

part_one(real_input)
part_two(real_input)
