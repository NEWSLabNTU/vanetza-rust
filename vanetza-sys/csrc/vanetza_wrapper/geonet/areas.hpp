#pragma once

#include <vanetza/geonet/areas.hpp>
#include <vanetza_wrapper/units/length.hpp>
#include <vanetza_wrapper/units/angle.hpp>

namespace vanetza_wrapper {
    namespace geonet {
        using vanetza::geonet::Area;
        using vanetza::geonet::CartesianPosition;
        using vanetza::geonet::GeodeticPosition;
        using vanetza_wrapper::units::LengthWrapper;
        using vanetza_wrapper::units::GeoAngleWrapper;

        struct AreaWrapper {
        public:
            AreaWrapper(Area&& src):
                m_inner(src)
            {}

        private:
            Area m_inner;
        };

        struct GeodeticPositionWrapper {
        public:
            GeodeticPositionWrapper(GeodeticPosition&& src):
                m_inner(src)
            {}

            GeodeticPositionWrapper(GeoAngleWrapper lat, GeoAngleWrapper lon):
                m_inner(GeodeticPosition(lat.m_inner, lon.m_inner))
            {}

            GeoAngleWrapper latitude() {
                return GeoAngleWrapper(m_inner.latitude);
            }

            GeoAngleWrapper longitude() {
                return GeoAngleWrapper(m_inner.longitude);
            }

        private:
            GeodeticPosition m_inner;
        };

        struct CartesianPositionWrapper {
        public:
            CartesianPositionWrapper(CartesianPosition&& src):
                m_inner(src)
            {}

            CartesianPositionWrapper(LengthWrapper x, LengthWrapper y):
                m_inner(CartesianPosition(x.m_inner, y.m_inner))
            {}

            LengthWrapper x() const {
                return LengthWrapper(m_inner.x);
            }

            LengthWrapper y() const {
                return LengthWrapper(m_inner.y);
            }

        private:
            CartesianPosition m_inner;
        };
    }
}
