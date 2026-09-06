#include <iostream>
#include <string>

class programmer {
public:
  unsigned int age;
  std::string name;
};

int main() {
  programmer Alexandre;
  programmer Joao;

  Alexandre.age = 23;
  Alexandre.name = "Alexandre Silvestri";

  Joao.age = 28;
  Joao.name = "Joao Silveira";

  std::cout << Alexandre.name << " has " << Alexandre.age << " years old!" << std::endl;
  std::cout << Joao.name << " has " << Joao.age << " year old!" << std::endl;

  return 0;
}
