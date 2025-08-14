#include "../headers/main.h"

int main() {
    printWelcome();
    
    int result = add(5, 3);
    std::cout << "5 + 3 = " << result << std::endl;
    
    std::cout << getMessage() << std::endl;
    
    return 0;
}

void printWelcome() {
    std::cout << "Welcome to the C++ Project!" << std::endl;
}

int add(int a, int b) {
    return a + b;
}

std::string getMessage() {
    return "This is a basic C++ template project.";
}