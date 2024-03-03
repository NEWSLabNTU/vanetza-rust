#pragma once

#include <vanetza/geonet/areas.hpp>
#include <vanetza_wrapper/units/length.hpp>
#include <vanetza_wrapper/units/angle.hpp>
#include <vanetza_wrapper/units/area.hpp>

namespace vanetza_wrapper {
    namespace geonet {
        using vanetza::geonet::Area;
        using vanetza::geonet::CartesianPosition;
        using vanetza::geonet::GeodeticPosition;
        using vanetza::geonet::Circle;
        using vanetza::geonet::Rectangle;
        using vanetza::geonet::Ellipse;
        using vanetza_wrapper::units::LengthWrapper;
        using vanetza_wrapper::units::AngleWrapper;
        using vanetza_wrapper::units::GeoAngleWrapper;
        using vanetza_wrapper::units::AreaWrapper;

        class GeonetAreaWrapper {
        public:
            GeonetAreaWrapper(const Area& src):
                m_inner(src)
            {}

            GeonetAreaWrapper(Area&& src):
                m_inner(src)
            {}

            const Area& inner_ref() const {
                return m_inner;
            }

            Area inner() const {
                return m_inner;
            }

        private:
            Area m_inner;
        };

        class GeodeticPositionWrapper {
        public:
            GeodeticPositionWrapper(const GeodeticPosition& src):
                m_inner(src)
            {}

            GeodeticPositionWrapper(GeodeticPosition&& src):
                m_inner(src)
            {}

            GeodeticPositionWrapper(const GeoAngleWrapper& lat, const GeoAngleWrapper& lon):
                m_inner(lat.m_inner, lon.m_inner)
            {}

            GeoAngleWrapper latitude() const {
                GeoAngleWrapper lat(m_inner.latitude);
                return lat;
            }

            GeoAngleWrapper longitude() const {
                GeoAngleWrapper lon(m_inner.longitude);
                return lon;
            }

            const GeodeticPosition& inner_ref() const {
                return m_inner;
            }

            GeodeticPosition inner() const {
                return m_inner;
            }

        private:
            GeodeticPosition m_inner;
        };

        class CartesianPositionWrapper {
        public:
            CartesianPositionWrapper(const CartesianPosition& src):
                m_inner(src)
            {}

            CartesianPositionWrapper(CartesianPosition&& src):
                m_inner(src)
            {}

            CartesianPositionWrapper(const LengthWrapper& x, const LengthWrapper& y):
                m_inner(x.m_inner, y.m_inner)
            {}

            LengthWrapper x() const {
                LengthWrapper x(m_inner.x);
                return x;
            }

            LengthWrapper y() const {
                LengthWrapper y(m_inner.y);
                return y;
            }

            const CartesianPosition& inner_ref() const {
                return m_inner;
            }

            CartesianPosition inner() const {
                return m_inner;
            }

        private:
            CartesianPosition m_inner;
        };

        class CircleWrapper {
        public:
            CircleWrapper(const Circle& src):
                m_inner(src)
            {}

            CircleWrapper(Circle&& src):
                m_inner(src)
            {}

            CircleWrapper(const LengthWrapper& r)
            {
                m_inner.r = r.m_inner;
            }

            LengthWrapper r() const {
                LengthWrapper r(m_inner.r);
                return r;
            }

            const Circle& inner_ref() const {
                return m_inner;
            }

            Circle inner() const {
                return m_inner;
            }

        private:
            Circle m_inner;
        };

        class RectangleWrapper {
        public:
            RectangleWrapper(const Rectangle& src):
                m_inner(src)
            {}

            RectangleWrapper(Rectangle&& src):
                m_inner(src)
            {}

            RectangleWrapper(const LengthWrapper& a, const LengthWrapper& b)
            {
                m_inner.a = a.m_inner;
                m_inner.b = b.m_inner;
            }

            LengthWrapper a() const {
                LengthWrapper a(m_inner.a);
                return a;
            }

            LengthWrapper b() const {
                LengthWrapper b(m_inner.b);
                return b;
            }

            const Rectangle& inner_ref() const {
                return m_inner;
            }

            Rectangle inner() const {
                return m_inner;
            }

        private:
            Rectangle m_inner;
        };

        class EllipseWrapper {
        public:
            EllipseWrapper(const Ellipse& src):
                m_inner(src)
            {}

            EllipseWrapper(Ellipse&& src):
                m_inner(src)
            {}

            EllipseWrapper(const LengthWrapper& a, const LengthWrapper& b)
            {
                m_inner.a = a.m_inner;
                m_inner.b = b.m_inner;
            }

            LengthWrapper a() const {
                LengthWrapper a(m_inner.a);
                return a;
            }

            LengthWrapper b() const {
                LengthWrapper b(m_inner.b);
                return b;
            }

            const Ellipse& inner_ref() const {
                return m_inner;
            }

            Ellipse inner() const {
                return m_inner;
            }

        private:
            Ellipse m_inner;
        };

        LengthWrapper distance(const GeodeticPosition& lhs, const GeodeticPosition& rhs) {
            auto distance = vanetza::geonet::distance(lhs, rhs);
            LengthWrapper output(distance);
            return output;
        }

        CartesianPosition canonicalize(const CartesianPosition& point, AngleWrapper azimuth) {
            auto pos = vanetza::geonet::canonicalize(point, azimuth.m_inner);
            return pos;
        }

        bool inside_or_at_border(const GeonetAreaWrapper& area, const GeodeticPosition& pos) {
            return vanetza::geonet::inside_or_at_border(area.inner_ref(), pos);
        }

        AreaWrapper area_size(const GeonetAreaWrapper& area) {
            auto size = vanetza::geonet::area_size(area.inner_ref());
            AreaWrapper output(size);
            return output;
        }
    }
}
