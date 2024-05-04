pub use warp::{reject::Rejection, reply::Reply, Filter};

pub trait Routable {
    fn menu_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone;
}
