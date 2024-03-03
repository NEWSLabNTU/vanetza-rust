#pragma once
#include <vanetza/geonet/areas.hpp>

using vanetza::geonet::CartesianPosition;

namespace vanetza_wrapper {
    namespace geonet {
        struct CartesianPositionWrapper {
        public:
            CartesianPositionWrapper(CartesianPosition&& src):
                m_inner(src)
            {}
            
        private:
            CartesianPosition m_inner;
        };
    }
}
