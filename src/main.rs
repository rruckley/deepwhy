use env_logger;
use log::info;

use deep_causality::prelude::*;

fn get_test_causaloid<'l>(id : IdentificationValue) -> BaseCausaloid<'l> {
    let description = "tests whether data exceeds threshold of 0.55";

    fn causal_fn(obs: NumericalValue) -> Result<bool, CausalityError> {
        Ok(true)
    }
    Causaloid::new(id, causal_fn, description)
}

fn main() {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    info!("Starting {pkg} v{ver}");

    
}
