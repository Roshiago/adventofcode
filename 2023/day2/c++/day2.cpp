#include <iostream>
#include <fstream>
#include <sstream>
#include <filesystem>

uint8_t is_ascii_number(uint8_t c) {
    return c >= '0' && c <= '9'; 
}

uint8_t parse_ascii_number(uint8_t c) {
    return c - '0';
}

void solve_puzzle(std::filesystem::path path) {
    std::ifstream file = std::ifstream(path);

    if (!file.is_open()) {
        std::cout << "[ERROR] File cannot be opened: " << path << "\n";
        return;
    }

    const size_t max_red = 12, max_green = 13, max_blue = 14;
    size_t part1_result = 0;
    size_t part2_result = 0;

    while (!file.eof()) {
        size_t game_number = 0;
        size_t in_one_red = 0, in_one_green = 0, in_one_blue = 0;
        size_t in_game_max_red = 0, in_game_max_green = 0, in_game_max_blue = 0;
        size_t red = 0, green = 0, blue = 0;
        std::string line;
        std::getline(file, line);
        std::cout << "---------------------\n";
        std::cout << "LINE: " << line << "\n";
        bool success_game = true;
        size_t colon_idx = line.find(':');
        for (size_t j = colon_idx - 1; j >= 0; j--) {
            if (line[j] == ' ') {
                std::string_view game = std::string_view(
                    line.begin() + j + 1, line.begin() + colon_idx - 1
                );
                game_number = std::atoi(game.data());
                break;
            }
        }
        size_t start_search = colon_idx;
        while (start_search < line.size()) {
            size_t possible_comma = line.find(',', start_search);
            size_t possible_semicolon = line.find(';', start_search);
            size_t possible_break = 0;
            bool has_reset = false;
            if (possible_comma < possible_semicolon) {
                possible_break = possible_comma;
            }
            else if (possible_semicolon < possible_comma) {
                has_reset = true;
                possible_break = possible_semicolon;
            }
            else {
                has_reset = true;
                possible_break = line.size();
            }
            std::string_view color;
            for (size_t j = possible_break - 1; j >= 0; j--) {
                if (line[j] == ' ') {
                    color = std::string_view(
                        line.begin() + j + 1, line.begin() + possible_break
                    );
                    break;
                }
            }
            
            size_t start_num = std::string::npos;
            size_t end_num = std::string::npos;
            for (size_t j = possible_break - 1; j >= start_search; j--) {
                if (is_ascii_number(line[j])) {
                    if (end_num == std::string::npos) end_num = j;
                    else start_num = j;
                }
            }
            if (start_num == std::string::npos) {
                start_num = end_num - 1;
            }
            size_t number = std::atoi(
                std::string_view(
                    line.begin() + start_num,
                    line.begin() + end_num
                ).data()
            );
            // std::cout << "PARSE? '" << number << "':'" << color << "'\n";

            if (color == "red") {
                // std::cout << "RED?\n";
                red += number;
                in_one_red += number;
            }
            if (color == "green") {
                // std::cout << "GREEN?\n";
                green += number;
                in_one_green += number;
            }
            if (color == "blue") {
                // std::cout << "BLUE?\n";
                blue += number;
                in_one_blue += number;
            }

            if (has_reset) {
                std::cout << "SET: " << game_number << " -> "
                          << in_one_red << "|" << in_one_green << "|" << in_one_blue
                          << "\n";
                success_game = success_game && (
                    in_one_red <= max_red
                    && in_one_green <= max_green
                    && in_one_blue <= max_blue
                );
                if (in_one_red > in_game_max_red) {
                    in_game_max_red = in_one_red;
                }
                if (in_one_green > in_game_max_green) {
                    in_game_max_green = in_one_green;
                }
                if (in_one_blue > in_game_max_blue) {
                    in_game_max_blue = in_one_blue;
                }
                in_one_red = in_one_green = in_one_blue = 0;
            }

            start_search = possible_break + 1;
        }
        if (success_game) {
            std::cout << "Game: " << game_number << "\n";
            part1_result += game_number;
        }
        size_t power_of_game = in_game_max_red * in_game_max_green * in_game_max_blue;
        std::cout << "Power of Game (" << game_number << ") is ["
                  << in_game_max_red << ":" << in_game_max_green << ":" << in_game_max_blue
                  << "]: " << power_of_game
                  << "\n";
        part2_result += power_of_game;
    }
    std::cout << "Result (Part 1): " << part1_result << "\n";
    std::cout << "Result (Part 2): " << part2_result << "\n";
    file.close();
}

int main() {
    solve_puzzle(std::filesystem::path("2023/day2/day2.txt"));
    return 0;
}