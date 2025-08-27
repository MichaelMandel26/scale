CFLAGS := "-Wall -Wextra -Wpedantic"
CFLAGS_OPTIMIZE := "-O2"

build:
  clang {{CFLAGS}} {{CFLAGS_OPTIMIZE}} scale.c -o build/scale

debug:
  clang {{CFLAGS}} -g scale.c -o build/scale
  lldb ./build/scale .

run: build
  ./build/scale

install:

