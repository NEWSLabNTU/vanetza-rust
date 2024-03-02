use autocxx::prelude::*;

include_cpp! {
    // #include "vanetza/common/manual_runtime.hpp"
    // #include "vanetza/common/runtime.hpp"
    // #include "vanetza/common/clock.hpp"
    #include "vanetza_wrapper.hpp"
    #include "vanetza/geonet/router.hpp"
    #include "vanetza/geonet/address.hpp"
    #include "vanetza/geonet/lifetime.hpp"
    #include "vanetza/geonet/traffic_class.hpp"
    #include "vanetza/geonet/data_request.hpp"
    #include "vanetza/geonet/data_confirm.hpp"
    #include "vanetza/geonet/areas.hpp"
    #include "vanetza/geonet/mib.hpp"

    safety!(unsafe_ffi)

    // vanetza/common/clock.hpp
    // generate!("vanetza::Clock")

    // vanetza/common/manual_runtime.hpp
    // generate!("vanetza::ManualRuntime")

    // vanetza/common/runtime.hpp
    // generate!("vanetza::Runtime")

    // vanetza/geonet/router.hpp
    generate!("vanetza::geonet::Router")

    // vanetza/geonet/address.hpp
    generate!("vanetza::geonet::Address")

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
    generate!("vanetza::geonet::DataConfirm")

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
}

pub use ffi::*;
