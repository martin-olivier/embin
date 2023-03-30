#include "asset.h"

#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

int main() {
    int fd = open("result", O_WRONLY | O_CREAT, 0644);
    if (fd < 0)
        return 1;

    write(fd, asset, asset_len);

    close(fd);
    return 0;
}