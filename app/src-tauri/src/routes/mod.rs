pub mod collections;
pub mod items;

use super::db;
use std::sync::Arc;

type RouterCtx = Arc<db::PrismaClient>;
type RouterBuilder = rspc::RouterBuilder<RouterCtx>;
pub type Router = rspc::Router<RouterCtx>;
