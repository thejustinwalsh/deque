pub mod collections;
pub mod items;

use super::db;
use rspc::Config;
use std::sync::Arc;

type RouterCtx = Arc<db::PrismaClient>;
type RouterBuilder = rspc::RouterBuilder<RouterCtx>;
pub type Router = rspc::Router<RouterCtx>;

pub fn mount() -> Arc<Router> {
    let config = Config::new();

    #[cfg(all(debug_assertions, test))]
    let config = config
        .export_ts_bindings("../src/hooks/rspc/bindings.ts")
        .set_ts_bindings_header("/* eslint-disable */");

    Router::new()
        .config(config)
        .merge("collection.", collections::mount())
        .merge("item.", items::mount())
        .build()
        .arced()
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_ts_bindings() {
        super::mount();
    }
}
