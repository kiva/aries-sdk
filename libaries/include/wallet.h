#ifndef __aries_sdk_wallet_h__
#define __aries_sdk_wallet_h__

typedef enum {
    SUCCESS = 0,
    ALREADY_EXISTS = 1,
    CANT_DELETE = 2,
    NOT_FOUND = 3
} aries_wallet_t;

/* Creates the wallet according `config` using `credentials` for access.
*
*  @param [in] `config` - a json string that details where to create the wallet
*  @param [in] `credentials` - a json string containing the necessary sign-in information to the wallet handler
*  @param [out] `error` - an error structure that can be checked if an error occurs. Includes a code and string message. If success, no further checking is needed
*  @return 1 - success, 0 otherwise
*/
extern int32_t aries_create_wallet(const char * const config,  const char * const credentials, const struct ExternError* error);

/* Opens a previously created wallet according `config` using `credentials` for access. 
*
*  @param [in] `config` - a json string that details where the wallet to be opened resides
*  @param [in] `credentials` - a json string containing the necessary sign-in information to the wallet handler
*  @param [out] `error` - an error structure that can be checked if an error occurs. Includes a code and string message. If success, no further checking is needed
*  @return wallet handle id, >0 - success, 0 otherwise
*/
extern uint32_t aries_open_wallet(const char* const config,  const char* const credentials, const struct ExternError* error);

/* Export a previously opened wallet according `export_config`
*
*  @param [in] `wallet_handle` - the currently opened wallet handle
*  @param [in] `export_config` - a json string containing the new config of the exported wallet
*  @param [out] `error` - an error structure that can be checked if an error occurs. Includes a code and string message. If success, no further checking is needed
*  @return 1 - success, 0 otherwise
*/
extern int32_t aries_export_wallet(uint32_t wallet_handle, const char* const export_config, const struct ExternError* error);

/* Import an existing wallet according `config` using `credentials` for access. The imported wallet will use the information in `import_config`
*
*  @param [in] `config` - a json string that details where the imported wallet will reside
*  @param [in] `credentials` - a json string containing the necessary sign-in information to the imported wallet handler
*  @param [in] `import_config` - a json string containing the config of the existing wallet from which to import
*  @param [out] `error` - an error structure that can be checked if an error occurs. Includes a code and string message. If success, no further checking is needed
*  @return 1 - success, 0 otherwise
*/
extern int32_t aries_import_wallet(const char* const config,  const char* const credentials, const char* const import_config, const struct ExternError* error);

/* Close a previously opened wallet
*
*  @param [in] `wallet_handle` - The wallet handle id
*  @return 1 - success, 0 otherwise
*/
extern int32_t aries_close_wallet(uint32_t wallet_handle);

/* Delete an existing wallet. It must be already closed if a file.
*
*  @param [in] `config` - a json string that details where the wallet to be deleted resides
*  @param [in] `credentials` - a json string containing the necessary sign-in information to the wallet handler
*  @return 1 - success, 0 otherwise
*/
extern int32_t aries_delete_wallet(const char * const config,  const char* const credentials, const struct ExternError* error);

#endif //aries_sdk_wallet_h
