use vanetza::{
    common::ManualRuntime,
    geonet::{mib::MIB, router::Router},
};

fn main() {
    let rt = ManualRuntime::default();
    let mib = MIB::default();
    let router = Router::new(&rt, &mib);
}
