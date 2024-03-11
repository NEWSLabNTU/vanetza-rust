#pragma once



#include <vanetza/btp/header.hpp>
#include <vanetza/geonet/address.hpp>
#include <vanetza/geonet/areas.hpp>
#include <vanetza/geonet/data_indication.hpp>
#include <vanetza/geonet/lifetime.hpp>
#include <vanetza/geonet/position_vector.hpp>
#include <vanetza/geonet/traffic_class.hpp>
#include <boost/optional.hpp>
#include <boost/variant.hpp>

namespace vanetza_wrapper
{

    
namespace btp
{
    using vanetza::btp::HeaderA;
    using vanetza::btp::HeaderB;
    using vanetza::btp::port_type;
    using vanetza::geonet::DataIndication;
    using vanetza::geonet::ShortPositionVector;
    using vanetza::geonet::TrafficClass;


struct BtpDataIndicationWrapper
{
    BtpDataIndicationWrapper();
    BtpDataIndicationWrapper(const vanetza::geonet::DataIndication&, const HeaderA&);
    BtpDataIndicationWrapper(const vanetza::geonet::DataIndication&, const HeaderB&);

    boost::optional<port_type> source_port;
    port_type destination_port;
    boost::optional<decltype(HeaderB::destination_port_info)> destination_port_info;
    decltype(vanetza::geonet::DataIndication::destination) destination;
    decltype(vanetza::geonet::DataIndication::its_aid) its_aid;
    decltype(vanetza::geonet::DataIndication::permissions) permissions;
    vanetza::geonet::ShortPositionVector source_position;
    vanetza::geonet::TrafficClass traffic_class;
    boost::optional<vanetza::geonet::Lifetime> remaining_packet_lifetime;
};

} // namespace btp
} // namespace vanetza

