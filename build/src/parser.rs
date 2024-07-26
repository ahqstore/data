use ahqstore_types::AHQStoreApplication;
use serde_json::{from_str, to_string};
use std::{
  fs::{self, File},
  io::Write,
};

struct Map {
  entries: usize,
  files: usize,
  c_file: File,
  search: File,
}

impl Map {
  fn new() -> Self {
    let _ = fs::create_dir_all("./db/map");
    let _ = fs::create_dir_all("./db/search");
    let _ = fs::create_dir_all("./db/apps");
    let _ = fs::create_dir_all("./db/dev");

    let mut file = File::create("./db/map/1.json").unwrap();
    let _ = file.write(b"{");

    let mut search = File::create("./db/search/1.json").unwrap();
    let _ = search.write(b"[");

    Self {
      entries: 0,
      files: 1,
      c_file: file,
      search,
    }
  }

  fn close_file(&mut self) {
    let _ = self.search.write_all(b"]");
    let _ = self.search.flush();
    let _ = self.c_file.write_all(b"}");
    let _ = self.c_file.flush();
  }

  fn new_file(&mut self) {
    self.files += 1;
    self.entries = 0;
    self.close_file();

    let mut map = File::create("./db/map/1.json").unwrap();
    let _ = map.write(b"{");

    let mut search = File::create("./db/map/1.json").unwrap();
    let _ = search.write(b"[");

    self.c_file = map;
    self.search = search;
  }

  fn add_author(&mut self, author: &str, app_id: &str) {
    let file = format!("./db/dev/{}", author);
    let mut val = fs::read_to_string(&file).unwrap_or("".to_string());
    val.push_str(&format!("{}\n", &app_id));

    let _ = fs::write(&file, val);
  }

  fn add(&mut self, app: AHQStoreApplication) {
    if self.entries >= 100_000 {
      self.new_file();
    }
    
    if self.entries > 0 {
      let _ = self.c_file.write(b",");
      let _ = self.search.write(b",");
    }

    self.add_author(&app.authorId, &app.appId);
    self.entries += 1;

    let _ = self
      .c_file
      .write(format!("\"{}\":\"{}\"", app.appDisplayName, app.appId).as_bytes());
    let _ = self.search.write(
      format!(
        "{{\"name:\": {:?}, \"title\": {:?}, \"id\": {:?}}}",
        app.appDisplayName, app.appShortcutName, app.appId
      )
      .as_bytes(),
    );

    let _ = fs::write(
      format!("./db/apps/{}.json", app.appId),
      to_string(&app).unwrap(),
    );
  }

  fn finish(mut self) {
    self.close_file();

    let _ = fs::write("./db/total", self.files.to_string());
  }
}

pub fn parser() {
  println!("⏲️ Please wait...");
  let _ = fs::remove_dir_all("./db");
  let _ = fs::create_dir_all("./db");
  let _ = fs::create_dir_all("./db/info");

  let _ = fs::copy("./home.keep.json", "./db/home.json");

  let mut map = Map::new();

  for users in fs::read_dir("./info").unwrap() {
    let user = users.unwrap();

    let _ = fs::copy(user.path(), format!("./db/info/{}", &user.file_name().to_string_lossy()));
  }
  for (uid, app) in fs::read_dir("./apps").unwrap().enumerate() {
    let app = app.unwrap();

    let data = fs::read_to_string(app.path()).unwrap();
    if let Ok(data) = from_str::<AHQStoreApplication>(&data) {
      println!(
        "{}. 🔎 {} -> {} ({})",
        uid + 1,
        &data.authorId,
        &data.appId,
        &data.appDisplayName
      );
      map.add(data);
    } else {
      println!("{}. ❌ Removing {}", uid + 1, app.file_name().to_string_lossy());
      let _ = fs::remove_file(app.path());
    }
  }
  map.finish();
  println!("✅ Done!");
}
