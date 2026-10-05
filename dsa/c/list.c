#include <stdio.h>
#include <stdlib.h>

typedef struct node {
    int number;
    struct node *next;
  } Node;

void printStoredNumbers(Node *head);

int main(void) {

  Node *list = NULL;
  int qty;

  printf("Type the quantity of numbers you want to store: ");
  scanf("%i", &qty);

  for (int i = 0; i < qty; i++) {
    Node *tmp = malloc(sizeof(Node));
    int current = i + 1;
    int carrier;
    printf("Type the number %i to store: ", current);
    scanf("%i", &carrier);
    tmp->number = carrier;
    tmp->next = list;
    list = tmp;
  }

  printStoredNumbers(list);

  return 0;
}

  void printStoredNumbers(Node *head) {
    Node *ptr = head;
    int counter = 1;

    while(ptr != NULL) {
    printf("Value on node %i is: %i\n", counter, ptr->number);
    ptr = ptr->next;
    counter++;
    };
  };

