#include <iostream>
#include <string>

int getStairSize(void);
std::string getSize(int size, char simbol);

int main() {
  const int layers = getStairSize();

  for (int i{1}; i < layers+1; i++) {
    std::string empties{getSize(layers-i, ' ')};
    std::string hashtags{getSize(i, '#')};

    std::cout << empties << hashtags << '\n';
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

std::string getSize(int size, char simbol) {
  return std::string(size, simbol);
}
