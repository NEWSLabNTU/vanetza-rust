#pragma once

#include <boost/units/systems/si/frequency.hpp>
#include <vanetza/units/frequency.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Frequency;

    class FrequencyWrapper {
    public:
        FrequencyWrapper(Frequency src) :
            m_inner(src)
        {}

        FrequencyWrapper(double hertz) :
            m_inner(hertz * boost::units::si::hertz)
        {}

        double as_hertz() const {
            return m_inner / boost::units::si::hertz;
        }

        Frequency m_inner;
    };
}
}
