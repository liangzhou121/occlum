#include "ocalls.h"
#include <errno.h>
#include <net/if.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <sys/vfs.h>
#include <fcntl.h>
#include <sys/mman.h>

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

int occlum_ocall_open_device(
    char *device_name_buf,
    size_t device_name_buf_len,
    uint32_t flags
) {
    int fd = open(device_name_buf, flags);
    // printf("open device %s, fd %d\n", device_name_buf, fd);
    return fd;
}

uint64_t occlum_ocall_device_mmap(
    uint64_t addr,
    size_t length,
    int prot,
    int flags,
    int fd,
    uint64_t offset
) {
    // printf("addr %x, length %d prot %d flags %d fd %d offset %lx ", addr, length, prot,
    //        flags, fd, offset);
    void *p = mmap((void *)addr, length, prot, flags, fd, offset);

    // void *p = mmap(NULL, length, PROT_READ | PROT_WRITE , MAP_SHARED, fd, offset);

    // if ( p != (void *) -1 ) {
    //     printf("ret %p\n", p);
    //     int *test_v = (int *)p;
    //     printf("try to acccess the mmapped address %x\n", *test_v);
    // }
    return (uint64_t)p;
}

int occlum_ocall_device_munmap(uint64_t addr, size_t length) {
    int ret = munmap((void *)addr, length);
    printf("munmap addr %x, size %x, ret %p\n", addr, length, ret);
    return 0;
}

int occlum_ocall_device_ioctl(
    int fd,
    int cmd,
    uint64_t arg
) {
    int ret = ioctl(fd, cmd, (void *)arg);

#if 0
    /* FIXME: very special case of DRM_IOCTL_I915_GETPARAM(I915_PARAM_HAS_BSD2): Intel Media
         *        Driver uses Sys-V IPC (semget and shmget family of syscalls) for multi-process
         *        user-mode synchronization (to load-balance execution of video encode/decode on
         *        two VCS rings) if I915_PARAM_HAS_BSD2 == true; we don't support shmget() in
         *        Gramine so we stub I915_PARAM_HAS_BSD2 = false; this leads to slightly worse
         *        performance because only one VCS ring is used for video encode/decode but this may
         *        be fixed after Media Driver removes this Sys-V IPC dependency (see comment
         *        https://bugzilla.mozilla.org/show_bug.cgi?id=1619585#c46). */
    if (cmd == /*DRM_IOCTL_I915_GETPARAM*/0xc0106446) {
        typedef struct drm_i915_getparam {int32_t param; int *value;} drm_i915_getparam_t;
        drm_i915_getparam_t *arg_getparam = (drm_i915_getparam_t *)arg;
        if (arg_getparam->param == /*I915_PARAM_HAS_BSD2*/31) {
            /* return BSD2 = false, meaning there is no second VCS ring */
            arg_getparam->value = 0;
        }
    }

    /* FIXME: another very special case, similar to the one above but using another IOCTL:
     *        DRM_IOCTL_I915_QUERY(DRM_I915_QUERY_ENGINE_INFO). Newer Intel Media Driver
     *        uses this IOCTL to collect info on all engines. Some of these engines are of
     *        class I915_ENGINE_CLASS_VIDEO (which corresponds to a VCS ring above). The code
     *        below filters these classes and only allows one to be returned (the others are
     *        marked as I915_ENGINE_CLASS_INVALID so Media Driver doesn't recognize them). */
    if (cmd == /*DRM_IOCTL_I915_QUERY*/0xc0106479) {
        struct i915_engine_class_instance {
            uint16_t engine_class;
            uint16_t engine_instance;
        };
        struct drm_i915_engine_info {
            struct i915_engine_class_instance engine;
            uint32_t rsvd0;
            uint64_t flags;
            uint64_t capabilities;
            uint64_t rsvd1[4];
        };
        struct drm_i915_query_engine_info {
            uint32_t num_engines;
            uint32_t rsvd[3];
            struct drm_i915_engine_info engines[];
        };
        struct drm_i915_query_item {
            uint64_t query_id;
            int32_t length;
            uint32_t flags;
            uint64_t data_ptr;
        };
        struct drm_i915_query {
            uint32_t num_items;
            uint32_t flags;
            struct drm_i915_query_item *query_items;
        };

        uint32_t engine_class_video_num = 0;
        struct drm_i915_query *arg_query = (struct drm_i915_query *)arg;
        for (uint32_t i = 0; i < arg_query->num_items; i++) {
            struct drm_i915_query_item *query_item = &arg_query->query_items[i];
            if (query_item->length <= 0 || query_item->data_ptr == 0) {
                continue;
            }
            if (query_item->query_id != /*DRM_I915_QUERY_ENGINE_INFO*/2 &&
                    query_item->query_id != /*new DRM_I915_QUERY_ENGINE_INFO*/0x1000d) {
                continue;
            }

            struct drm_i915_query_engine_info *query_engine_info =
                (struct drm_i915_query_engine_info *)query_item->data_ptr;
            for (uint32_t j = 0; j < query_engine_info->num_engines; j++) {
                struct drm_i915_engine_info *engine_info = &query_engine_info->engines[j];
                if (engine_info->engine.engine_class == /*I915_ENGINE_CLASS_VIDEO*/2) {
                    engine_class_video_num++;
                    if (engine_class_video_num > 1) {
                        /* this is second, third, ... 'video' engine, mark as invalid */
                        engine_info->engine.engine_class = /*I915_ENGINE_CLASS_INVALID*/ -1;
                    }
                }
            }
        }
    }
#endif
    return ret;
}

