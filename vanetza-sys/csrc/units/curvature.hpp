#pragma once

// #include <boost/units/systems/si/curvature.hpp>
#include <vanetza/units/curvature.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Curvature;
    using vanetza::units::reciprocal_metre;

    class CurvatureWrapper {
    public:
        CurvatureWrapper(Curvature src) :
            m_inner(src)
        {}

        double as_reciprocal_meters() {
            return m_inner / reciprocal_metre;
        }

    private:
        Curvature m_inner;
    };
}
}
