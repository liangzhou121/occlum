
#include <sys/ioctl.h>
#include <fcntl.h>
#include <stdint.h>

#include "dcap.h"

int get_dcap_quote_size()
{
    int sgx_fd = 0;
    if ((sgx_fd = open("/dev/sgx", O_RDONLY)) < 0)
    {
        return DEVICE_ERROR;
    }

    int quote_size = 0;
    if (ioctl(sgx_fd, SGXIOC_GET_DCAP_QUOTE_SIZE, &quote_size) < 0)
    {
        return DEVICE_IOCTL_ERROR;
    }

    return quote_size;
}

int generate_dcap_quote(sgx_report_data_t *report_data, void *dcap_buffer, int length)
{
    int sgx_fd = 0;
    int quote_size = length;
    sgxioc_gen_dcap_quote_arg_t gen_quote_arg = {
        .report_data = report_data,
        .quote_len = (uint32_t *)&quote_size,
        .quote_buf = dcap_buffer};

    if ((sgx_fd = open("/dev/sgx", O_RDONLY)) < 0)
    {
        return DEVICE_ERROR;
    }

    if (ioctl(sgx_fd, SGXIOC_GEN_DCAP_QUOTE, &gen_quote_arg) < 0)
    {
        return DEVICE_IOCTL_ERROR;
    }

    return 0;
}