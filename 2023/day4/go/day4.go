package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"unicode"
)

func part1() {
    file, err := os.Open("./day4.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    result := 0
    for scanner.Scan() {
        line := scanner.Text();
        winning_idx := -1;
        colon_idx := -1
        chars := []rune(line);
        for i := 0; i < len(chars) ; i++ {
            if chars[i] == ':' {
                colon_idx = i;
            }
            if chars[i] == '|' {
                winning_idx = i;
                break
            }
        }

        winning_numbers := map[string]bool {};
        buf := "";
        for i := colon_idx; i <= winning_idx; i++ {
            if unicode.IsDigit(chars[i]) {
                buf += string(chars[i]);
            } else {
                if len(buf) > 0 {
                    winning_numbers[buf] = true;
                    buf = "";
                }
            }
        }
        fmt.Println(winning_numbers)
        local_result := 0;
        buf = "";
        for i := winning_idx; i < len(chars) ; i++ {
            if unicode.IsDigit(chars[i]) {
                buf += string(chars[i]);
            } else {
                fmt.Print(buf, " ");
                if winning_numbers[buf] {
                    if local_result == 0 {
                        local_result = 1;
                    } else {
                        local_result *= 2;
                    }
                }
                buf = "";
            }
        }
        fmt.Print(buf, " ");
        if winning_numbers[buf] {
            if local_result == 0 {
                local_result = 1;
            } else {
                local_result *= 2;
            }
        }
        buf = "";
        fmt.Println("\n", local_result);
        result += local_result;
    }
    fmt.Println(result);

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
}

func main() {
    part1()
}