#include <iostream>

int calcAge(int day, int month, int year, int currentDay, int currentMonth, int currentYear);

int main() {
  int age{calcAge(22, 5, 2003, 7, 9, 2026)};
  std::cout << age << std::endl;
  return 0;
}

int calcAge(int day, int month, int year, int currentDay, int currentMonth, int currentYear) {
  int approxAge = currentYear - year;
  
  if (currentMonth >= month) {
    return approxAge;
  } else {
    return approxAge--;
  }  
}
