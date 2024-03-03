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
        
        double as_seconds() {
            return m_inner / boost::units::si::second;
        }
        
    private:
        Duration m_inner;
    };
}
}
