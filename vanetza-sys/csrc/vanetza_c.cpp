#include <cstdint>
#include <vanetza/common/manual_runtime.hpp>
#include <vanetza/common/position_provider.hpp>
#include <vanetza/common/byte_buffer.hpp>
#include <vanetza/btp/port_dispatcher.hpp>
#include <vanetza/access/interface.hpp>
#include <vanetza/dcc/data_request.hpp>
#include <vanetza/dcc/interface.hpp>
#include <vanetza/net/mac_address.hpp>
#include <vanetza/net/cohesive_packet.hpp>
#include <vanetza/net/ethernet_header.hpp>
#include <vanetza/geonet/mib.hpp>
#include <vanetza/geonet/router.hpp>
#include "vanetza_c.hpp"

namespace vn = vanetza;
namespace gn = vanetza::geonet;

uint64_t mac_address_to_u64(vn::MacAddress &mac);

c_mib mib_new() {
    return new gn::MIB();
}

void mib_del(c_mib self) {
    delete reinterpret_cast<gn::MIB*>(self);
}

c_router router_new(c_runtime runtime, c_mib mib) {
    return new gn::Router(*reinterpret_cast<vn::Runtime*>(runtime),
                          *reinterpret_cast<gn::MIB*>(mib));
}

void router_del(c_router self) {
    delete reinterpret_cast<gn::Router*>(self);
}

c_runtime manual_runtime_new() {
    return new vn::ManualRuntime();
}

void runtime_del(c_runtime self) {
    delete reinterpret_cast<vn::Runtime*>(self);
}

c_guc_data_request guc_data_request_new(c_mib mib, uint32_t its_aid) {
    auto mib_ = reinterpret_cast<gn::MIB*>(mib);
    return new gn::GucDataRequest(*mib_, its_aid);
}

void guc_data_request_del(c_guc_data_request self) {
    delete reinterpret_cast<gn::GucDataRequest*>(self);
}

c_gbc_data_request gbc_data_request_new(c_mib mib, uint32_t its_aid) {
    auto mib_ = reinterpret_cast<gn::MIB*>(mib);
    return new gn::GbcDataRequest(*mib_, its_aid);
}

void gbc_data_request_del(c_gbc_data_request self) {
    delete reinterpret_cast<gn::GbcDataRequest*>(self);
}

c_gac_data_request gac_data_request_new(c_mib mib, uint32_t its_aid) {
    auto mib_ = reinterpret_cast<gn::MIB*>(mib);
    return new gn::GacDataRequest(*mib_, its_aid);
}

void gac_data_request_del(c_gac_data_request self) {
    delete reinterpret_cast<gn::GacDataRequest*>(self);
}

c_shb_data_request shb_data_request_new(c_mib mib, uint32_t its_aid) {
    auto mib_ = reinterpret_cast<gn::MIB*>(mib);
    return new gn::ShbDataRequest(*mib_, its_aid);
}
void shb_data_request_del(c_shb_data_request self) {
    delete reinterpret_cast<gn::ShbDataRequest*>(self);
}

c_tsb_data_request tsb_data_request_new(c_mib mib, uint32_t its_aid) {
    auto mib_ = reinterpret_cast<gn::MIB*>(mib);
    return new gn::TsbDataRequest(*mib_, its_aid);
}

void tsb_data_request_del(c_tsb_data_request self) {
    delete reinterpret_cast<gn::TsbDataRequest*>(self);
}


c_address address_new1(uint64_t mac_addr) {
    vn::MacAddress mid = vn::create_mac_address(mac_addr);
    return new gn::Address(mid);
}

c_address address_new2(bool manually_configured, uint16_t station_type,
                       uint16_t country_code, uint64_t mac_addr) {
    vn::MacAddress mid = vn::create_mac_address(mac_addr);

    auto ptr = new gn::Address(mid);
    ptr->is_manually_configured(manually_configured);
    ptr->station_type(static_cast<gn::StationType>(station_type));
    ptr->country_code(country_code);
    return ptr;
}

void address_del(c_address self) {
    delete reinterpret_cast<gn::Address*>(self);
}

bool address_get_is_manually_configured(c_address self) {
    auto addr = reinterpret_cast<gn::Address*>(self);
    return addr->is_manually_configured();
}

uint16_t address_get_station_type(c_address self) {
    auto addr = reinterpret_cast<gn::Address*>(self);
    auto st = addr->station_type();
    return static_cast<uint16_t>(st);
}

uint16_t address_get_country_code(c_address self) {
    auto addr = reinterpret_cast<gn::Address*>(self);
    auto code = addr->country_code();
    auto raw = code.raw();
    return static_cast<uint16_t>(raw);
}

uint64_t address_get_mid(c_address self) {
    auto addr = reinterpret_cast<gn::Address*>(self);
    auto mid = addr->mid();
    auto value = mac_address_to_u64(mid);
    return value;
}

uint64_t mac_address_to_u64(vn::MacAddress &mac) {
    uint64_t value = 0;
    for (std::size_t i = 0; i < 6; ++i) {
        value |= mac.octets[i] << (8 * (5 - i));
    }
    return value;
}
