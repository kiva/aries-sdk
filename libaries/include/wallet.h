//
// Created by Cam Parra on 2019-09-18.
//

#ifndef aries_sdk_wakket_h
#define aries_sdk_wakket_h


#ifdef __cplusplus
extern "C" {
#endif

extern int32_t aries_create_wallet(const char * const config,  const char * const credentials);
extern int32_t aries_open_wallet(const char * const config,  const char * const credentials);
extern int32_t aries_export_wallet(int32_t wallet_handle, const char * export_config);
extern int32_t aries_import_wallet(const char * config,  const char * credentials, const char * const import_config);
extern int32_t aries_close_wallet(int32_t wallet_handle);
extern int32_t aries_delete_wallet(const char * const config,  const char * const credentials);
//extern int32_t aries_generate_wallet_key(const char * const config);

#ifdef __cplusplus
}
#endif

#endif //aries_sdk_wakket_h
