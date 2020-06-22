use std::path::PathBuf;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/hello/<file..>")]
pub fn mult_seg(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}