#include <iostream>
#include <string>
#include <vector>

class Base {
public:
    virtual ~Base() {}
    virtual void execute() = 0;
};

class DerivedA : public Base {
public:
    void execute() override {
        std::cout << "Executing DerivedA" << std::endl;
    }
};

class DerivedB : public Base {
public:
    void execute() override {
        std::cout << "Executing DerivedB" << std::endl;
    }
};

class DerivedC : public Base {
public:
    void execute() override {
        std::cout << "Executing DerivedC" << std::endl;
    }
};

void vtable_example() {
    std::vector<Base*> objects = { new DerivedA(), new DerivedB(), new DerivedC() };
    for (auto obj : objects) {
        obj->execute();
        delete obj;
    }
}

// Jump table example function
void jump_table_example(int option) {
    switch (option) {
        case 1: std::cout << "Option 1 selected" << std::endl; break;
        case 2: std::cout << "Option 2 selected" << std::endl; break;
        case 3: std::cout << "Option 3 selected" << std::endl; break;
        case 4: std::cout << "Option 4 selected" << std::endl; break;
        default: std::cout << "Invalid option" << std::endl; break;
    }
}

// Regular function calls to demonstrate call sequences
void function_one() {
    std::cout << "Function One" << std::endl;
}

void function_two() {
    std::cout << "Function Two" << std::endl;
}

void function_three() {
    std::cout << "Function Three" << std::endl;
}

void call_sequence_example() {
    function_one();
    function_two();
    function_three();
}

// A simple recursive function for disassembler testing
int recursive_function(int n) {
    if (n <= 0) return 0;
    return n + recursive_function(n - 1);
}

int main() {
    // Test the vtable functionality
    std::cout << "Testing vtable example:" << std::endl;
    vtable_example();

    // Test the jump table example
    std::cout << "\nTesting jump table example:" << std::endl;
    for (int i = 1; i <= 5; ++i) {
        jump_table_example(i);
    }

    // Test the call sequence
    std::cout << "\nTesting call sequence example:" << std::endl;
    call_sequence_example();

    // Test recursive function
    std::cout << "\nTesting recursive function:" << std::endl;
    int result = recursive_function(5);
    std::cout << "Result of recursive_function(5): " << result << std::endl;

    return 0;
}
