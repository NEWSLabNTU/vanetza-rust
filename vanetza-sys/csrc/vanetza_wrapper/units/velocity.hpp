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

        VelocityWrapper(double meter_per_second) :
            m_inner(meter_per_second * boost::units::si::meter_per_second)
        {}

        double as_meters_per_second() const {
            return m_inner / boost::units::si::meter_per_second;
        }

        Velocity m_inner;
    };

    class NauticalVelocityWrapper {
    public:
        NauticalVelocityWrapper(NauticalVelocity src) :
            m_inner(src)
        {}

        NauticalVelocityWrapper(double knot) :
            m_inner(knot * vanetza::units::metric::knot)
        {}

        double as_knots() const {
            return m_inner / vanetza::units::metric::knot;
        }

        NauticalVelocity m_inner;
    };
}
}
