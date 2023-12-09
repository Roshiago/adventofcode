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

        max_idx, max_num = -1, -1

        min_idx, min_num = len(line), len(line)
        for wnum in num2digit:
            left_idx = line.find(wnum)
            right_idx = line.rfind(wnum)
            if left_idx == -1 and right_idx == -1:
                continue
            
            pull = []
            if left_idx != -1:
                pull.append(left_idx)
            
            if right_idx != -1:
                pull.append(right_idx)

            for wnum_idx in pull:
                if max_idx < wnum_idx:
                    max_idx = wnum_idx
                    max_num = num2digit[wnum]
                
                if min_idx > wnum_idx:
                    min_idx = wnum_idx
                    min_num = num2digit[wnum]

        for idx, c in enumerate(line):
            if not c.isdigit():
                continue

            if min_idx is not None and idx < min_idx:
                min_idx = idx
                min_num = int(c)
            elif min_idx is None:
                min_idx = idx
                min_num = int(c)

            if max_idx is not None and idx > max_idx:
                max_idx = idx
                max_num = int(c)
            elif max_idx is None:
                max_idx = idx
                max_num = int(c)

        value = min_num * 10 + max_num
        result += value

    return result


with open("day1.txt") as f:
    puzzle = f.readlines()

result = parse_puzzle(puzzle)
print("RESULT: ", result)
