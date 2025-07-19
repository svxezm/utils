#include <stdio.h>
#include <time.h>

#define GMT (-3)

char* get_week_day(int day) {
  char* days[7] = {
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
  };

  if (day > 6 || day < 0) {
    return "unknown day";
  }
  return days[day];
}

int main() {
  time_t rawtime;
  struct tm* info;

  time(&rawtime);
  info = gmtime(&rawtime);

  char* wday = get_week_day(info->tm_wday);
  int day = info->tm_mday;
  int month = info->tm_mon;
  int year = info->tm_year + 1900;
  int hours = info->tm_hour + GMT;
  int minutes = info->tm_min;

  printf(
    "📅 Today is: %s, %02d/%02d/%04d - %02d:%02d ⏳\n",
    wday, day, month, year, hours, minutes
  );
  
  return 0;
}
