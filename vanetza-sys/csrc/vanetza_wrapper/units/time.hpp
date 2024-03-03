#pragma once

#include <boost/units/systems/si/time.hpp>
#include <vanetza/units/time.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Duration;

    class DurationWrapper {
    public:
        DurationWrapper(Duration src) :
            m_inner(src)
        {}

        DurationWrapper(double seconds) :
            m_inner(seconds * boost::units::si::second)
        {}

        double as_seconds() const {
            return m_inner / boost::units::si::second;
        }

        Duration m_inner;
    };
}
}
