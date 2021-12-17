use colored::Colorize;

pub fn init() {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}\n{}\n",
        (format_args!(
          "{} {} {}",
          chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
          record.level(),
          record.target(),
        ))
        .to_string()
        .bright_black(),
        message,
      ))
    })
    .level(log::LevelFilter::Info)
    .level_for("rmw", log::LevelFilter::Trace)
    .chain(std::io::stdout())
    // .chain(fern::log_file("output.log")?)
    .apply()
    .unwrap();
}
