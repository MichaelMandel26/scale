#include <stdio.h>

static void print_usage(FILE *stream, int exit_code);
static int walk(const char *fpath, const struct stat *sb, int typeflag, struct FTW *ftwbuf);
static bool get_path_size(char *path);
char* readable_fs(double size, char *buf);
