use confy;

pub fn open_file() {
  let cfg: ConfigData = confy::load("odam_config").unwrap();
  println!("{:#?}", cfg);
}
