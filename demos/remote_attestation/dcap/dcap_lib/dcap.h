#ifndef _DCAP_H_
#define _DCAP_H_

#include <stdint.h>
#include <sgx_ql_quote.h>

#define DEVICE_ERROR -1
#define DEVICE_IOCTL_ERROR -2

/** @struct sgxioc_gen_dcap_quote_arg_t
   *  A structure for DCAP quote generation
   *
   *  @var report_data
   *    The input report data to be included in the quote.
   *  @var quote_len
   *    A value-result argument: the caller must initialize it to contain the
   *    size (in bytes) of the buffer pointed to by quote_buf; on return it
   *    will contain the actual size of the output quote.
   *  @var quote_buf
   *    A pointer to the buffer to store the output quote.
*/
typedef struct {
    sgx_report_data_t *report_data;
    uint32_t          *quote_len;
    uint8_t           *quote_buf;
} sgxioc_gen_dcap_quote_arg_t;

#define SGXIOC_GET_DCAP_QUOTE_SIZE        _IOR('s', 7, uint32_t)
#define SGXIOC_GEN_DCAP_QUOTE             _IOWR('s', 8, sgxioc_gen_dcap_quote_arg_t)

int get_dcap_quote_size();
int generate_dcap_quote(sgx_report_data_t *report_data, void *dcap_buffer, int length);
#endif //_DCAP_H_