#include <stdio.h>
#include <memory.h>
#include <string.h>
#include <stdint.h>

#define DEBUG 0
#define dprintf(args...) \
    do { \
        if (DEBUG) printf(args); \
    } while (0)

#define BUF_SIZE 255
#define TRUE 1
#define FALSE 0

// Numbers and shift
static const char *char_numbers = "one\0two\0three\0four\0five\0six\0seven\0eight\0nine";
static const int shift_numbers[] = { 0, 4, 8, 14, 19, 24, 28, 34, 40 };

int get_num_by_shift(uint32_t shift);
int solve_puzzle(const char * filepath);
uint8_t is_ascii_number(uint8_t c);
uint8_t parse_ascii_number(uint8_t c);
uint8_t try_parse_number(const char *buf, int *out_number);

int main() {
    int result = solve_puzzle("./day1.txt");
    printf("RESULT: %d\n", result);
    return 0;
}

int solve_puzzle(const char * filepath) {
    char line[BUF_SIZE] = {};
    char buf[BUF_SIZE] = {};

    char c = '\0';
    const uint8_t min_length = 3;

    uint32_t result = 0;
    uint32_t length = 0;
    FILE *file = fopen(filepath, "r");
    if (file == NULL) {
        printf("[ERROR] Cannot open the file with path: %s\n", filepath);
        return -1;
    }
    while (c != EOF) {
        c = fgetc(file);
        if (c != '\n' && c != EOF) {
            line[length++] = c;
            continue;
        }
        int left_number = 0;
        for (uint32_t i = 0; i < length; i++) {
            buf[i] = line[i];
            if (strlen(buf) >= min_length) {
                dprintf("STRAIGHT BUF: %s\n", buf);
                uint8_t found = try_parse_number(buf, &left_number);
                if (found) {
                    break;
                }
            }

            if (is_ascii_number(line[i])) {
                dprintf("FOUND STRAIGHT NUM: %c\n", line[i]);
                left_number = parse_ascii_number(line[i]);
                break;
            }
        }
        memset(buf, 0, BUF_SIZE);

        int right_number = 0;
        for (int i = length - 1; i >= 0; i--) {
            buf[i] = line[i];
            if (strlen(buf + i) >= min_length){
                dprintf("BACKWARD BUF: %s\n", buf + i);
                uint8_t found = try_parse_number(buf + i, &right_number);
                if (found) {
                    break;
                }
            }

            if (is_ascii_number(line[i])) {
                dprintf("FOUND BACKWARD NUM: %c\n", line[i]);
                right_number = parse_ascii_number(line[i]);
                break;
            }
        }
        memset(buf, 0, BUF_SIZE);

        int actual_number = left_number * 10 + right_number;
        dprintf("Num in line (%s): %d\n", line, actual_number);
        result += actual_number;
        length = 0;
        memset(line, 0, length);
    }

    fclose(file);
    return result;
}

int get_num_by_shift(uint32_t shift) {
    switch (shift) {
        case 0: return 1;
        case 4: return 2;
        case 8: return 3;
        case 14: return 4;
        case 19: return 5;
        case 24: return 6;
        case 28: return 7;
        case 34: return 8;
        case 40: return 9;
        default: break;
    }
    return -1;
}

uint8_t is_ascii_number(uint8_t c) {
    return c >= '0' && c <= '9'; 
}

uint8_t parse_ascii_number(uint8_t c) {
    return c - '0';
}

uint8_t try_parse_number(const char *buf, int *out_number) {
    uint8_t has_num = FALSE;
    for (uint8_t j = 0; j < 9; j++) {
        int shift = shift_numbers[j];
        const char *current_num = char_numbers + shift;
        dprintf("CHECK NUM: %s\n", current_num);
        has_num = strstr(buf, current_num) != NULL;

        if (has_num) {
            dprintf("FOUND NUM: %s\n", buf);
            *out_number = get_num_by_shift(shift);
            break;
        }
    }
    return has_num;
}
