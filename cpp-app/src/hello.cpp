#include <iostream>
extern "C" void rust_func();

int main(int argc, char* argv[]) {
    std::cout << "Hello C++!" << std::endl;
    rust_func();
    return 0;
}