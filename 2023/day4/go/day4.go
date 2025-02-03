package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"unicode"
	"math"
)

func parse_number(chars []rune) int32 {
	var card_num int32 = 0;

	for i := 0; i < len(chars); i++ {
		if unicode.IsDigit(chars[i]) {
			card_num = (chars[i] - '0') + card_num*10;
		}
	}

    return card_num;
}


type Pair struct {
	card_num, winning_count interface{}
}


func parse_card(chars []rune) Pair {
    winning_idx := -1;
    colon_idx := -1

    for i := 0; i < len(chars) ; i++ {
        if chars[i] == ':' {
            colon_idx = i;
        }
        if chars[i] == '|' {
            winning_idx = i;
            break
        }
    }

	card_num := parse_number(chars[0:colon_idx+1]);

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

    var local_result int32 = 0;
    buf = "";
    for i := winning_idx; i < len(chars) ; i++ {
        if unicode.IsDigit(chars[i]) {
            buf += string(chars[i]);
			continue;
        }

        if winning_numbers[buf] {
			local_result += 1; 
        }

        buf = "";
    }

    if winning_numbers[buf] {
		local_result += 1; 
    }

	result := Pair {card_num, local_result};

	return result;
}

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

		card_pair := parse_card([]rune(line));
		winning_count := card_pair.winning_count.(int32);

		var local_result float64 = 0;
		if (winning_count > 0) {
			local_result = math.Pow(2, float64(winning_count - 1));
	    }

        result += int(local_result);
    }
	fmt.Println("Part 1:", result);

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
}


func part2() {
    file, err := os.Open("./day4.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

	saved_cards := map[int32]Pair {};

    for scanner.Scan() {
        line := scanner.Text();

		card_pair := parse_card([]rune(line));
		card_num := card_pair.card_num.(int32);

		saved_cards[card_num] = card_pair;
    }

	result := 0;

	queue := []Pair {};

	for _, card := range saved_cards {
		result += 1;
		winning_count := card.winning_count.(int32);

		if (winning_count == 0) {
			continue;
		}

		queue = append(queue, card);
	}

	for len(queue) > 0 {
		card := queue[0];
		queue = queue[1:];

		card_num := card.card_num.(int32);
		winning_count := card.winning_count.(int32);

	    var i int32 = 1;
	    for ; i <= winning_count; i++ {
	    	next_card_idx := card_num + i;
	    	next_card := saved_cards[next_card_idx];

			result += 1;

	    	if (next_card.winning_count.(int32) > 0) {
	    		queue = append(queue, next_card);
			}
	    }
	}

	fmt.Println("Part 2:", result);

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
}

func main() {
    part1();
	part2();
}
