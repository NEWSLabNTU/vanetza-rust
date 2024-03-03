#pragma once

#include <boost/units/systems/si/plane_angle.hpp>
#include <boost/units/systems/angle/degrees.hpp>
#include <vanetza/units/angle.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Angle;
    using vanetza::units::GeoAngle;
    using vanetza::units::TrueNorth;
    using vanetza::units::true_north_degrees;

    class AngleWrapper {
    public:
        AngleWrapper(Angle& src) :
            m_inner(src)
        {}

        double as_radians() {
            return m_inner / boost::units::si::radian;
        }

    private:
        Angle m_inner;
    };

    class GeoAngleWrapper {
    public:
        GeoAngleWrapper(GeoAngle& src) :
            m_inner(src)
        {}

        double as_degrees() {
            return m_inner / boost::units::degree::degree;
        }

    private:
        GeoAngle m_inner;
    };

    class TrueNorthWrapper {
    public:
        TrueNorthWrapper(TrueNorth src) :
            m_inner(src)
        {}

        double as_degrees() {
            TrueNorth zero(0.0 * true_north_degrees);
            return (m_inner - zero) / boost::units::degree::degree;
        }

    private:
        TrueNorth m_inner;
    };
}
}
