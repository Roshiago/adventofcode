import typing as ty


def parse_puzzle(puzzle: ty.Sequence[str]) -> int:
    result = 0
    num2digit = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    for line in puzzle:
        value = None

        right_idx, right_number = -1, -1
        left_idx, left_number = 99999, 99999

        for char_num in num2digit:
            left_find_idx = line.find(char_num)
            right_find_idx = line.rfind(char_num)
            if left_find_idx == -1 and right_find_idx == -1:
                continue

            pull = []
            if left_find_idx != -1:
                pull.append(left_find_idx)

            if right_find_idx != -1:
                pull.append(right_find_idx)

            for char_num_idx in pull:
                if right_idx <= char_num_idx:
                    right_idx = char_num_idx
                    right_number = num2digit[char_num]

                if left_idx >= char_num_idx:
                    left_idx = char_num_idx
                    left_number = num2digit[char_num]

        for idx, c in enumerate(line):
            if not c.isdigit():
                continue

            if left_idx is not None and idx < left_idx:
                left_idx = idx
                left_number = int(c)
            elif left_idx is None:
                left_idx = idx
                left_number = int(c)

            if right_idx is not None and idx > right_idx:
                right_idx = idx
                right_number = int(c)
            elif right_idx is None:
                right_idx = idx
                right_number = int(c)

        value = left_number * 10 + right_number
        result += value

    return result


def main():
    with open("day1.txt") as f:
        puzzle = f.readlines()

    result = parse_puzzle(puzzle)
    print("RESULT: ", result)


if __name__ == "__main__":
    main()
