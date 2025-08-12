build:
  clang -Wall -Wextra -Wpedantic -O2 scale.c -o build/scale

debug:
  clang -Wall -Wextra -Wpedantic -g scale.c -o build/scale
  lldb ./build/scale .

run: build
  ./build/scale
