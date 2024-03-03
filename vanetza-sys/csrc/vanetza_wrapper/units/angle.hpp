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
        AngleWrapper(Angle src) :
            m_inner(src)
        {}

        AngleWrapper(double radians) :
            m_inner(radians * boost::units::si::radian)
        {}

        double as_radians() const {
            return m_inner / boost::units::si::radian;
        }

        Angle m_inner;
    };

    class GeoAngleWrapper {
    public:
        GeoAngleWrapper(GeoAngle src) :
            m_inner(src)
        {}

        GeoAngleWrapper(double degrees) :
            m_inner(degrees * boost::units::degree::degree)
        {}

        double as_degrees() const {
            return m_inner / boost::units::degree::degree;
        }

        GeoAngle m_inner;
    };

    class TrueNorthWrapper {
    public:
        TrueNorthWrapper(TrueNorth src) :
            m_inner(src)
        {}

        TrueNorthWrapper(double degrees) :
            m_inner(degrees * true_north_degrees)
        {}

        double as_degrees() const {
            TrueNorth zero(0.0 * true_north_degrees);
            return (m_inner - zero) / boost::units::degree::degree;
        }

        TrueNorth m_inner;
    };
}
}
