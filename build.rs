use vergen::{Config, vergen};

fn main() {
  // Generate the default 'cargo:' instruction output
  let mut config = Config::default();
  *config.git_mut().enabled_mut() = false;
  let _ = vergen(config);
  let _ = vergen(Config::default());
}
