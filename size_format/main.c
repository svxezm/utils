#include <stdio.h>

int main() {
  double size;
  char* units[] = { "b", "kb", "mb", "gb", "tb", "pb" };
  int unitIndex = 0;
  int unitsSize = sizeof(units) / sizeof(units[0]);

  printf("Insert a number of bytes to convert: ");
  scanf("%lf", &size);

  while (size >= 1024.0 && unitIndex < unitsSize) {
    size /= 1024.0;
    unitIndex++;
  }

  printf("%.2lf %s\n", size, units[unitIndex]);

  return 0;
}
