use vanetza::{
    common::runtime::Runtime,
    geonet::{mib::MIB, router::Router},
};

fn main() {
    let rt = Runtime::new_manual_runtime();
    let mib = MIB::default();

    // TODO: not implemented yet
    // let _router = Router::new(&rt, &mib);
}
