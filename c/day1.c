#include <stdio.h>
#include <memory.h>
#include <string.h>

#define DEBUG 0
#define dprintf(args...) \
    do { \
        if (DEBUG) printf(args); \
    } while (0)\


int get_num_by_shift(int shift) {
    // const char *char_numbers = "one two three four five six seven eight nine";
    switch (shift)
    {
    case 0: return 1;
    case 4: return 2;
    case 8: return 3;
    case 14: return 4;
    case 19: return 5;
    case 24: return 6;
    case 28: return 7;
    case 34: return 8;
    case 40: return 9;
    default:
        break;
    }
    return -1;
}

int parse_puzzle(const char * filepath) {
    char c = '\0';
    char line[255] = {};
    char buf[255] = {};
    int number = -1;
    int zero_ascii = 48;
    int nine_ascii = 57;
    const char *char_numbers = "one\0two\0three\0four\0five\0six\0seven\0eight\0nine";
    const int numbers[] = { 0, 4, 8, 14, 19, 24, 28, 34, 40 };

    int result = 0;
    int length = 0;
    int linenum = 0;
    FILE *file = fopen(filepath, "r");
    if (file == NULL) {
        printf("[ERROR] Cannot open the file with path: %s\n", filepath);
        return -1;
    }
    while (c != EOF)
    {
        c = fgetc(file);
        if (c != '\n' && c != EOF) {
            line[length++] = c;
            continue;
        }
        for (int i = 0; i < length; i++) {
            buf[i] = line[i];
            dprintf("STRAIGHT BUF: %s\n", buf);
            int found = -1;
            for (int j = 0; j < 9; j++) {
                int shift = numbers[j];
                const char *current_num = char_numbers + shift;
                dprintf("CHECK NUM: %s:(%ld)\n", current_num, strlen(current_num));
                if (
                    strstr(buf, current_num) != NULL
                    && strlen(buf) >= strlen(current_num)
                ) {
                    dprintf("FOUND STRAIGHT CHAR: %s\n", buf);
                    number = get_num_by_shift(shift);
                    memset(buf, 0, i);
                    found = 1;
                    break;
                }
            }

            if (found != -1) {
                break;
            }

            if (line[i] >= zero_ascii && line[i] <= nine_ascii) {
                dprintf("FOUND STRAIGHT NUM: %c\n", line[i]);
                number = line[i] - zero_ascii;
                memset(buf, 0, i);
                break;
            }
        }

        for (int i = length - 1; i >= 0; i--) {
            buf[i] = line[i];
            int found = -1;
            dprintf("BACKWARD BUF: %s\n", buf + i);
            for (int j = 0; j < 9; j++) {
                int shift = numbers[j];
                const char *current_num = char_numbers + shift;
                dprintf("CHECK NUM: %s:(%ld)\n", current_num, strlen(current_num));
                if (
                    strstr(buf + i, current_num) != NULL
                    && strlen(buf + i) >= strlen(current_num)
                ) {
                    dprintf("FOUND BACKWARD CHAR: %s\n", buf + i);
                    number = number * 10 + get_num_by_shift(shift);
                    memset(buf, 0, 255);
                    found = 1;
                    break;
                }
            }

            if (found != -1) {
                break;
            }

            if (line[i] >= zero_ascii && line[i] <= nine_ascii) {
                dprintf("FOUND BACKWARD NUM: %c\n", line[i]);
                number = number * 10 + (line[i] - zero_ascii);
                memset(buf, 0, 255);
                break;
            }
        }
        dprintf("Num in line (%d): %d\n", linenum, number);
        result += number;
        memset(line, 0, length);
        length = 0;
        linenum += 1;
        number = 0;
    }

    fclose(file);
    return result;
}

int main() {
    int result = parse_puzzle("./day1.txt");
    printf("RESULT: %d\n", result);
    return 0;
}