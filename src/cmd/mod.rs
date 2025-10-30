mod builds_project;
pub mod handler;
pub mod make;
mod respond_with_generated_page;
mod respond_with_generated_page_holder;
pub mod serve;
mod service;
mod service_manager;
mod value_parser;
pub mod watch;

const STATIC_FILES_PUBLIC_PATH: &str = "assets";
