#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int count_total_len(char* str) {
  int size = 1;
  for (int i = 1; i < strlen(str); i++) {
    if (str[i] == ' ' && str[i - 1] != ' ') size++;
  }
  return size;
}

void prepare_chars(char* buffer, char str[], int len) {
  int count = 0;
  int chcount = 0;
  char ch[9] = {0};

  for (int i = 0; i <= len; i++) {
    if (str[i] == 48 || str[i] == 49) {
      ch[chcount++] = str[i];
    } else if ((isspace(str[i]) || i == len) && (chcount > 0)) {
      buffer[count++] = (char)strtol(ch, 0, 2);
      memset(ch, 0, sizeof(ch));
      chcount = 0;
    }
  }
  buffer[count] = '\0';
}

int main() {
  char* bin = "01101000 01100101 01101100 01101100 01101111";
  int stringlen = strlen(bin);
  int fnlen = count_total_len(bin);
  char* chars = (char*)calloc(fnlen + 1, sizeof(long));

  prepare_chars(chars, bin, stringlen);
  
  printf("===\n\n%s\n\n===\n", chars);
  free(chars);
  return 0;
}
