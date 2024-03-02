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

