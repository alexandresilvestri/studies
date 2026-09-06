#include <iostream>
#include <string>

float sumNumbers(int a, int b) {
  return a + b;
}

int main() {
  double preco{19.99};
  int centavos{static_cast<int>(preco * 100)};

  std::string sumToString{std::to_string(centavos)};

  std::cout << sumToString << std::endl;

  return 0;
}

