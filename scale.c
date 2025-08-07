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

int main(int argc, char *argv[]) {
  program_name = argv[0];

  char *file = NULL;

  const char *short_opts = "hl";
  const struct option long_opts[] = {{"help", no_argument, NULL, 'h'},
                                     {"list", no_argument, NULL, 'l'},
                                     {NULL, 0, NULL, 0}};

  int opt;
  while (optind < argc) {
    if ((opt = getopt_long(argc, argv, short_opts, long_opts, NULL)) != -1) {
      switch (opt) {
      case 'h':
        print_usage(stdout, EXIT_SUCCESS);
        break;
      case 'l':
        list_mode = true;
        break;
      default:
        print_usage(stdout, EXIT_FAILURE);
        break;
      }
    } else {
      file = argv[optind];
      optind++;
    }
  }

  if (!file) {
    print_usage(stdout, EXIT_FAILURE);
  }

  bool is_success = get_path_size(file);
  if (!is_success) {
    perror("Failed to get path size");
    return EXIT_FAILURE;
  }

  printf("Total size: %lld\n", size);

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
      printf("%lld\t%s\n", sb->st_size, fpath);
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
