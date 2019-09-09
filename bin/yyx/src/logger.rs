pub fn setup_logger() -> Result<(), fern::InitError> {
  #[cfg(debug_assertions)]
  {
    pretty_env_logger::init();
  }

  #[cfg(not(debug_assertions))]
  {
    fern::Dispatch::new()
      .format(|out, message, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
        ))
      })
      .level(log::LevelFilter::Trace)
      .chain(std::io::stdout())
      .chain(fern::log_file("yyx.log")?)
      .apply()?;
  }
  Ok(())
}
