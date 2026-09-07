#include <iostream>
#include <string>

int getStairSize(void);
std::string getSize(int size, int layers, char simbol);

int main() {
  const int layers = getStairSize();

  for (int i{1}; i < layers+1; i++) {
    std::string empties{getSize(layers-i, layers, ' ')};
    std::string hashtags{getSize(i, layers, '#')};

    std::cout << empties << hashtags << std::endl;
  }

  return 0;
}

int getStairSize(void) {
  int number{};
  
  do {
    std::cout << "Type how many layer the stair you want to have: ";
    std::cin >> number;
  } while(number < 0);

  return number;
}

std::string getSize(int size, int layers, char simbol) {
  int counter{1};
  const char value{simbol};
  std::string stack{};

    for (int k{0}; k < size; k++) {
      stack += value;
    }

  return stack;
}
