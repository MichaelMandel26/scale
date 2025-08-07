#define _XOPEN_SOURCE 700
#include <errno.h>
#include <ftw.h>
#include <getopt.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

#include "scale.h"

static const char *program_name;
static off_t size = 0;
static bool list_mode = false;

static const char *short_opts = "hl";
static const struct option long_opts[] = {{"help", no_argument, NULL, 'h'},
                                          {"list", no_argument, NULL, 'l'},
                                          {NULL, 0, NULL, 0}};

int main(int argc, char *argv[]) {
  program_name = argv[0];
  int opt;

  while ((opt = getopt_long(argc, argv, short_opts, long_opts, NULL)) != -1) {
    switch (opt) {
    case 'h':
      print_usage(stdout, EXIT_SUCCESS);
      break;
    case 'l':
      list_mode = true;
      break;
    default:
      print_usage(stderr, EXIT_FAILURE);
    }
  }

  if (optind == argc) {
    fprintf(stderr, "Error: missing <file> argument\n");
    print_usage(stderr, EXIT_FAILURE);
  }
  if (optind < argc - 1) {
    fprintf(stderr, "Error: too many arguments\n");
    print_usage(stderr, EXIT_FAILURE);
  }

  char *file = argv[optind];

  bool is_success = get_path_size(file);
  if (!is_success) {
    perror("Failed to get path size");
    return EXIT_FAILURE;
  }

  char buf[10];
  printf("Total size: %s\n", readable_fs(size, buf));

  return EXIT_SUCCESS;
}

void print_usage(FILE *stream, int exit_code) {
  fprintf(stream,
          "Usage: %s [OPTIONS] <file>\n"
          "  -h, --help         Display this help and exit\n"
          "  -l, --list         Enable verbose mode\n",
          program_name);
  exit(exit_code);
}

static int walk(const char *fpath, const struct stat *sb, int typeflag,
                struct FTW *ftwbuf) {
  (void)ftwbuf;
  if (typeflag == FTW_F) {
    size += sb->st_size;
    if (list_mode) {
      char buf[10];
      printf("%s\t%s\n", readable_fs(sb->st_size, buf), fpath);
    }
  }
  return 0;
}

static bool get_path_size(char *path) {
  struct stat sb;
  if (stat(path, &sb) < 0) {
    perror(path);
    return false;
  }

  if (S_ISREG(sb.st_mode)) {
    size = sb.st_size;
    return true;
  } else if (S_ISDIR(sb.st_mode)) {
    if (nftw(path, &walk, 20, FTW_PHYS | FTW_DEPTH) < 0) {
      return false;
    }
    return true;
  }

  return false;
}
char *readable_fs(double size, char *buf) {
  int i = 0;
  const char *units[] = {"B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"};
  while (size > 1024) {
    size /= 1024;
    i++;
  }
  sprintf(buf, "%.*f %s", i, size, units[i]);
  return buf;
}
