#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

int main(int argc, char **argv) {
    int fd;
    char buf[4096];
    ssize_t n;
    
    if (argc < 2) {
        while ((n = read(0, buf, sizeof(buf))) > 0) {
            write(1, buf, n);
        }
        return 0;
    }
    
    for (int i = 1; i < argc; i++) {
        fd = open(argv[i], O_RDONLY);
        if (fd < 0) {
            write(2, "neko: ", 6);
            write(2, argv[i], strlen(argv[i]));
            write(2, ": No such file\n", 16);
            continue;
        }
        while ((n = read(fd, buf, sizeof(buf))) > 0) {
            write(1, buf, n);
        }
        close(fd);
    }
    return 0;
}