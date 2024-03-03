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

        double as_meters_per_second_squared() {
            return m_inner / boost::units::si::meter_per_second_squared;
        }

    private:
        Acceleration m_inner;
    };
}
}
