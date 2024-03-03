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

        double as_hertz() {
            return m_inner / boost::units::si::hertz;
        }

    private:
        Frequency m_inner;
    };
}
}
