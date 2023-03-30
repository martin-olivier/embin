#include "asset.h"

#include <stdio.h>
#include <fcntl.h>

#if (defined(_WIN32) || defined(_WIN64))
#define OPEN(path, flags, mode) _open(path, flags, mode)
#define CLOSE(fd) _close(fd)
#else
#include <unistd.h>
#define OPEN(path, flags, mode) open(path, flags, mode)
#define CLOSE(fd) close(fd)
#endif

int main() {
    int fd = OPEN("result", O_WRONLY | O_CREAT, 0644);
    if (fd < 0)
        return 1;

    write(fd, asset, asset_len);

    CLOSE(fd);
    return 0;
}