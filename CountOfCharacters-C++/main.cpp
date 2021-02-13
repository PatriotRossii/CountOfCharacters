#include <iostream>
#include <string>
#include <windows.h>

int main() {
    SetConsoleOutputCP(65001);

    std::string string;
    char symbol;

    std::cout << "Введите строку: ";
    std::getline(std::cin, string);

    std::cout << "Введите символ для поиска: ";
    std::cin >> symbol;

    int result = 0;
    int position = 0;

    while(int new_pos = string.find(symbol, position) != std::string::npos) {
        ++result;
        position += new_pos + 1;
    }
    std::cout << "Данный символ встречается в строке " << result << " раз";
}