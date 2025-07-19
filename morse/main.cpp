#include <iostream>
#include <vector>
using namespace std;

string characters[] = {
  ".-",      // A
  "-...",    // B
  "-.-.",    // C
  "-..",     // D
  ".",       // E
  "..-.",    // F
  "--.",     // G
  "....",    // H
  "..",      // I
  ".---",    // J
  "-.-",     // K
  ".-..",    // L
  "--",      // M
  "-.",      // N
  "---",     // O
  ".--.",    // P
  "--.-",    // Q
  ".-.",     // R
  "...",     // S
  "-",       // T
  "..-",     // U
  "...-",    // V
  ".--",     // W
  "-..-",    // X
  "-.--",    // Y
  "--..",    // Z
  
  "-----",   // 0
  ".----",   // 1
  "..---",   // 2
  "...--",   // 3
  "....-",   // 4
  ".....",   // 5
  "-....",   // 6
  "--...",   // 7
  "---..",   // 8
  "----.",   // 9

  "-.-.--",  // ! 33
  "--..--",  // , 44
  "-....-",  // - 45
  ".-.-.-",  // . 46
  "-..-.",   // / 47
  "---...",  // : 58
  "-.-.-.",  // ; 59
  "-...-",   // = 61
  "..--..",  // ? 63

  
  /* 
  "..--..",  // ? 63
  "-.-.--",  // ! 33
  ".-.-.-",  // . 46
  "--..--",  // , 44
  "-.-.-.",  // ; 59
  "---...",  // : 58
  ".-.-.",   // + 43
  "-....-",  // - 45
  "-..-.",   // / 47
  "-...-",   // = 61
   */
};

int getCharCode(int i) {
  int character;
  
  if (i < 26) {
    character = 'A' + i;
  } else if (i >= 26 && i <= 36) {
    character = '0' + (i - 26);
  } else {
    switch (i) {
      case 37: character = '!'; break;
      case 38: character = ','; break;
      case 39: character = '-'; break;
      case 40: character = '.'; break;
      case 41: character = '/'; break;
      case 42: character = ':'; break;
      case 43: character = ';'; break;
      case 44: character = '='; break;
      case 45: character = '?'; break;
    }
  }

  cout << i << ": " << character << endl;
  return character;
}

void binaryToAlpha(string* result, const string source) {
  vector<string> chars;
  string signs = "";
  string word = "";

  for (int i = 0; i < source.size(); i++) {
    if (source[i] != ' ') {
      signs += source[i];
    } else {
      chars.push_back(signs);
      signs = "";
    }
  }
  word += '\0';

  for (int i = 0; i < chars.size(); i++) {
    if (chars[i] == "/") {
      word += ' ';
      continue;
    }

    for (int j = 0; j < sizeof(characters) / sizeof(characters[0]); j++) {
      if (chars[i] == characters[j]) {
        // int index = getCharCode(j);
        word += getCharCode(j);
      }
    }
  }
  *result = word;
}

int main() {
  string result = "";
  const string source = "--- .. / - ..- -.. --- / -... . -- ----- -...- ";

  binaryToAlpha(&result, source);

  for (int i = 0; i < result.size(); i++) {
    cout << result[i];
  }
  cout << endl;

  return 0;
}
