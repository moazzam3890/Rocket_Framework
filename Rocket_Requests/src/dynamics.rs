#[get("/<name>/<age>/<gender>")]
pub fn dyn_path(name: String, age: u8, gender:bool) -> String {
    if gender == true {
        format!("{} : {} : Male", name, age)
    } else {
        format!("{}: {} : Female", name, age)
    }
}