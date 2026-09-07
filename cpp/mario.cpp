#include <iostream>
#include <string>

int getStairSize(void);
std::string getEmptySize(int size, int layers);
std::string getHashtagSize(int size, int layers);

int main() {
  const int layers = getStairSize();

  for (int i{1}; i < layers+1; i++) {
    std::string empties{getEmptySize(layers-i, layers)};
    std::string hashtags{getHashtagSize(i, layers)};

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

std::string getEmptySize(int size, int layers) { 
  const char empty{' '};
  std::string empties{};

  for (int j{0}; j < size; j++) {
    empties += empty;
  }

  return empties;
}

std::string getHashtagSize(int size, int layers) {
  int hashtagCounter{1};
  const char hashtag{'#'};
  std::string hashtags{};

    for (int k{0}; k < size; k++) {
      hashtags += hashtag;
    }

  return hashtags;
}
