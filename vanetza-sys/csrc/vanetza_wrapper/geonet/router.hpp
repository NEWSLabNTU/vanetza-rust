#pragma once

#include <vanetza/geonet/router.hpp>
#include <vanetza/net/mac_address.hpp>

namespace vanetza_wrapper
{
namespace geonet
{
    using vanetza::geonet::Router;
    using vanetza::MacAddress;

    class RouterRef {
    public:
        RouterRef(Router& ref) :
            m_inner(ref)
        {}

        void indicate(std::unique_ptr<PacketVariantWrapper> packet, const MacAddress& sender, const MacAddress& destination) {
            m_inner.indicate(std::move(packet->m_inner), sender, destination);
        }
        
    private:
        Router& m_inner;
    };
}
}
