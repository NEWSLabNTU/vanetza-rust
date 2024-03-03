#pragma once

#include <boost/units/systems/si/velocity.hpp>
#include <vanetza/units/velocity.hpp>

namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Velocity;
    using vanetza::units::NauticalVelocity;

    class VelocityWrapper {
    public:
        VelocityWrapper(Velocity src) :
            m_inner(src)
        {}

        double as_meters_per_second() {
            return m_inner / boost::units::si::meter_per_second;
        }

    private:
        Velocity m_inner;
    };

    class NauticalVelocityWrapper {
    public:
        NauticalVelocityWrapper(NauticalVelocity src) :
            m_inner(src)
        {}

        double as_knots() {
            return m_inner / vanetza::units::metric::knot;
        }

    private:
        NauticalVelocity m_inner;
    };
}
}
