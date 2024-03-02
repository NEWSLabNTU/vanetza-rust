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
