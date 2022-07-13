#include "ocalls.h"
#include <errno.h>
#include <net/if.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/vfs.h>
#include <fcntl.h>

void occlum_ocall_sync(void) {
    sync();
}

int occlum_ocall_ioctl_repack(int fd, int request, char *buf, int len, int *recv_len) {
    int ret = 0;

    switch (request) {
        case SIOCGIFCONF:
            if (recv_len == NULL) {
                errno = EINVAL;
                return -1;
            }

            struct ifconf config = { .ifc_len = len, .ifc_buf = buf };
            ret = ioctl(fd, SIOCGIFCONF, &config);
            if (ret == 0) {
                *recv_len = config.ifc_len;
            }
            break;

        default:
            errno = EINVAL;
            return -1;
    }

    return ret;
}

int occlum_ocall_ioctl(int fd, int request, void *arg, size_t len) {
    if (((arg == NULL) ^ (len == 0)) == 1) {
        errno = EINVAL;
        return -1;
    }

    return ioctl(fd, request, arg);
}

int occlum_ocall_statfs(const char *path, struct statfs *buf) {
    return statfs(path, buf);
}

int occlum_open_i915() {
    return open("/dev/dri/card0", O_RDONLY);
}

int occlum_open_render() {
    return open("/dev/dri/renderD128", O_RDONLY);
}

uint64_t occlum_device_mmap(
    uint64_t addr,
    size_t length,
    int prot,
    int flags,
    int fd,
    uint64_t offset
) {
    return (uint64_t)mmap((void *)addr, length, prot, flags, fd, offset);
}