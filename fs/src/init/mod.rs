mod log;

pub fn init() {
  log::init();
  std::thread::spawn(|| {
    rmw_utf8::init();
  });
}
