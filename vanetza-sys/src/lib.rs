use autocxx::prelude::*;

include_cpp! {
    // #include "vanetza/common/manual_runtime.hpp"
    // #include "vanetza/common/runtime.hpp"
    // #include "vanetza/common/clock.hpp"
    #include "vanetza/common/its_aid.hpp"
    #include "vanetza/common/position_fix.hpp"
    #include "vanetza/net/mac_address.hpp"
    #include "vanetza/net/chunk_packet.hpp"
    #include "vanetza/net/cohesive_packet.hpp"
    #include "vanetza/net/packet_variant.hpp"
    #include "vanetza_wrapper.hpp"
    #include "vanetza/geonet/router.hpp"
    #include "vanetza/geonet/address.hpp"
    #include "vanetza/geonet/lifetime.hpp"
    #include "vanetza/geonet/traffic_class.hpp"
    #include "vanetza/geonet/data_request.hpp"
    #include "vanetza/geonet/data_confirm.hpp"
    #include "vanetza/geonet/areas.hpp"
    #include "vanetza/geonet/mib.hpp"
    #include "vanetza/geonet/station_type.hpp"
    #include "vanetza/geonet/position_vector.hpp"
    #include "vanetza/geonet/transport_interface.hpp"
    // #include "vanetza/geonet/timestamp.hpp"
    // #include "vanetza/geonet/location_table.hpp"
    #include "vanetza/geonet/interface.hpp"
    #include "vanetza/geonet/data_indication.hpp"

    safety!(unsafe_ffi)

    // vanetza/common/clock.hpp
    // generate!("vanetza::Clock")

    // vanetza/common/manual_runtime.hpp
    // generate!("vanetza::ManualRuntime")

    // vanetza/common/runtime.hpp
    // generate!("vanetza::Runtime")

    // vanetza/common/its_aid.hpp
    generate_ns!("vanetza::aid")
    generate_pod!("vanetza::ItsAid")

    // vanetza/common/position_fix.hpp
    generate!("vanetza::PositionFix")

    // vanetza/net/mac_address.hpp
    generate_pod!("vanetza::MacAddress")

    // vanetza/net/chunk_packet.hpp
    generate!("vanetza::ChunkPacket")

    // vanetza/net/cohesive_packet.hpp
    generate!("vanetza::CohesivePacket")

    // vanetza/net/packet_variant.hpp
    generate!("vanetza::PacketVariant")

    // vanetza/geonet/router.hpp
    generate!("vanetza::geonet::Router")

    // vanetza/geonet/station_type.hpp
    generate_pod!("vanetza::geonet::StationType")

    // vanetza/geonet/address.hpp
    generate_pod!("vanetza::geonet::Address")

    // vanetza/geonet/lifetime.hpp
    generate_pod!("vanetza::geonet::Lifetime")

    // vanetza/geonet/traffic_class.hpp
    generate_pod!("vanetza::geonet::TrafficClass")

    // vanetza/geonet/data_request.hpp
    generate!("vanetza::geonet::DataRequest")
    generate!("vanetza::geonet::DataRequestWithArea")
    generate!("vanetza::geonet::DataRequestWithAddress")
    generate!("vanetza::geonet::GucDataRequest")
    generate!("vanetza::geonet::GbcDataRequest")
    generate!("vanetza::geonet::GacDataRequest")
    generate!("vanetza::geonet::ShbDataRequest")
    generate!("vanetza::geonet::TsbDataRequest")

    // vanetza/geonet/data_confirm.hpp
    generate_pod!("vanetza::geonet::DataConfirm")
    generate_pod!("vanetza::geonet::DataConfirm_ResultCode")

    // vanetza/geonet/areas.hpp
    generate!("vanetza::geonet::Area")
    generate!("vanetza::geonet::Circle")
    generate!("vanetza::geonet::Rectangle")
    generate!("vanetza::geonet::Ellipse")
    // generate!("vanetza::geonet::CartesianPosition")
    // generate!("vanetza::geonet::GeodeticPosition")

    // vanetza/geonet/mib.hpp
    generate!("vanetza::geonet::ManagementInformationBase")
    generate_pod!("vanetza::geonet::AddrConfMethod")
    generate_pod!("vanetza::geonet::InterfaceType")
    generate_pod!("vanetza::geonet::SecurityDecapHandling")
    generate_pod!("vanetza::geonet::UnicastForwarding")
    generate_pod!("vanetza::geonet::BroadcastForwarding")

    // vanetza/geonet/position_vector.hpp
    generate!("vanetza::geonet::ShortPositionVector")
    generate!("vanetza::geonet::LongPositionVector")

    // vanetza/geonet/timestamp.hpp
    // generate!("vanetza::geonet::Timestamp")

    // vanetza/geonet/location_table.hpp
    // generate!("vanetza::geonet::LocationTable")

    // vanetza/geonet/transport_interface.hpp
    generate!("vanetza::geonet::TransportInterface")

    // vanetza/geonet/interface.hpp
    generate_pod!("vanetza::geonet::UpperProtocol")
    generate_pod!("vanetza::geonet::TransportType")

    // vanetza/geonet/data_indication.hpp
    generate!("vanetza::geonet::DataIndication")
}

pub use ffi::*;
