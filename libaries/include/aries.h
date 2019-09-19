#ifndef __aries_sdk_included__
#define __aries_sdk_included__

#include <stdint.h>

struct ExternError {
    int32_t code;
    char* message;
};

#ifdef __cplusplus
extern "C" {
#endif

extern int32_t aries_sdk_free_string(char* input);

// Put other header file includes here
#include "wallet.h"

#ifdef __cplusplus
}
#endif

#endif