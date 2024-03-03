#pragma once

#include <boost/units/systems/si/angular_velocity.hpp>
#include <vanetza/units/angular_velocity.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::AngularVelocity;

    class AngularVelocityWrapper {
    public:
        AngularVelocityWrapper(AngularVelocity src) :
            m_inner(src)
        {}

        AngularVelocityWrapper(double radians_per_second) :
            m_inner(radians_per_second * boost::units::si::radians_per_second)
        {}

        double as_radians_per_second() const {
            return m_inner / boost::units::si::radians_per_second;
        }

        AngularVelocity m_inner;
    };
}
}
