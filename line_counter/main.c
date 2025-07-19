#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define IN  1
#define OUT 0

void trim(char* text) {
  int start = 0, end = strlen(text) - 1;

  while (isspace(text[start])) {
    start++;
  }

  while (end > start && isspace(text[end])) {
    end--;
  }

  if (start > 0 || end < (strlen(text) - 1)) {
    memmove(text, text + start, end - start + 1);
    text[end - start + 1] = '\0';
  }
}

int main() {
  printf("Please, insert the target path:\n");

  char path[500];
  fgets(path, 500, stdin);
  size_t has_path = strlen(path) > 2;

  if (!has_path) {
    printf("Target file not informed. Using text.txt...\n");
  }

  trim(path);
  char* target = has_path ? path : "text.txt";

  FILE* file = fopen(target, "r");
  if (file == NULL) {
    perror("File not found.");
    return 1;
  }

  unsigned int lines = 0, words = 0, chars = 0;
  int state = OUT;
  char ch;

  while ((ch = fgetc(file)) != EOF) {
    chars++;
    if (ch == '\n') lines++;

    if (!isspace(ch)) {
      state = OUT;
    } else if (state == OUT) {
      state = IN;
      words++;
    }
  }

  fclose(file);
  printf("File: %s\n\nLines: %d\nWords: %d\nChars: %d\n", target, lines, words, chars);
  
  return 0;
}
