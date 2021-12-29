pub mod about;
pub mod home;
mod page_not_found;
mod pi_calc;

pub use self::{about::About, home::Home, page_not_found::PageNotFound, pi_calc::PiCalc};
