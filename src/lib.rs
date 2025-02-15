pub mod commands;
pub mod config;
pub mod control_protocol;
pub mod daemon;
mod global_rpc;
mod haven_util;
pub mod socket;
pub mod stream;

fn log_error<E>(label: &str) -> impl FnOnce(E) + '_
where
    E: std::fmt::Debug,
{
    move |s| log::warn!("{label} restart, error: {:?}", s)
}
