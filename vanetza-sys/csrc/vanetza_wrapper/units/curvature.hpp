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

        CurvatureWrapper(double reciprocal_metre_value) :
            m_inner(reciprocal_metre_value * reciprocal_metre)
        {}

        double as_reciprocal_meters() const {
            return m_inner / reciprocal_metre;
        }

        Curvature m_inner;
    };
}
}
