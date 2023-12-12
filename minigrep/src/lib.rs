use std::error::Error;

pub mod config;
pub mod file;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    file::File::new(&cfg.filename)?
        .search(&cfg.pattern, cfg.ignore_case)?
        .print();
    Ok(())
}
