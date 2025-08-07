build:
  clang -Wall -Wextra -Wpedantic -O2 scale.c -o scale

debug:
  clang -Wall -Wextra -Wpedantic -g scale.c -o scale
  lldb ./scale .

run: build
  ./scale
