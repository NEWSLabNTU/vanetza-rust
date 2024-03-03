#pragma once

#include <boost/units/systems/si/acceleration.hpp>
#include <vanetza/units/acceleration.hpp>

namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Acceleration;

    class AccelerationWrapper {
    public:
        AccelerationWrapper(Acceleration src) :
            m_inner(src)
        {}

        AccelerationWrapper(double meters_per_second_squared) :
            m_inner(meters_per_second_squared * boost::units::si::meter_per_second_squared)
        {}

        double as_meters_per_second_squared() const {
            return m_inner / boost::units::si::meter_per_second_squared;
        }

        Acceleration m_inner;
    };
}
}
