#pragma once
#include <cstdint>

#ifdef __cplusplus
  #define EXPORT_C extern "C"
#else
  #define EXPORT_C
#endif


#ifdef __cplusplus
// C++ code here
#endif


// Opaque pointer type alias for C-lang
typedef void* c_mib;
typedef void* c_router;
typedef void* c_runtime;
typedef void* c_manual_runtime;
typedef void* c_clock;
typedef void* c_guc_data_request;
typedef void* c_gac_data_request;
typedef void* c_gbc_data_request;
typedef void* c_shb_data_request;
typedef void* c_tsb_data_request;
typedef void* c_address;

// C functions
EXPORT_C c_mib mib_new();
EXPORT_C void mib_del(c_mib self);

EXPORT_C c_router router_new(c_runtime runtime, c_mib mib);
EXPORT_C void router_del(c_router self);

EXPORT_C c_manual_runtime manual_runtime_new();
EXPORT_C void runtime_del(c_runtime self);

EXPORT_C c_guc_data_request guc_data_request_new(c_mib mib, uint32_t its_aid);
EXPORT_C void guc_data_request_del(c_guc_data_request self);

EXPORT_C c_gbc_data_request gbc_data_request_new(c_mib mib, uint32_t its_aid);
EXPORT_C void gbc_data_request_del(c_gbc_data_request self);

EXPORT_C c_gac_data_request gac_data_request_new(c_mib mib, uint32_t its_aid);
EXPORT_C void gac_data_request_del(c_gac_data_request self);

EXPORT_C c_shb_data_request shb_data_request_new(c_mib mib, uint32_t its_aid);
EXPORT_C void shb_data_request_del(c_shb_data_request self);

EXPORT_C c_tsb_data_request tsb_data_request_new(c_mib mib, uint32_t its_aid);
EXPORT_C void tsb_data_request_del(c_tsb_data_request self);

EXPORT_C c_address address_new1(uint64_t mac_addr);
EXPORT_C c_address address_new2(bool manually_configured, uint16_t station_type, uint16_t country_code, uint64_t mac_addr);
EXPORT_C void address_del(c_address self);
EXPORT_C bool address_get_is_manually_configured(c_address self);
EXPORT_C uint16_t address_get_station_type(c_address self);
EXPORT_C uint16_t address_get_country_code(c_address self);
EXPORT_C uint64_t address_get_mid(c_address self);
