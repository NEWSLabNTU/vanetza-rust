use vanetza_sys::vanetza::geonet::LocationTable as CxxLocationTable;

pub struct LocationTableRef<'a> {
    pub(crate) ref_: &'a CxxLocationTable,
}
