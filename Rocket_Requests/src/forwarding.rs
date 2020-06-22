use rocket::http::RawStr;

#[get("/<name>/<age>/<gender>", rank = 3)]
pub fn user_sus(name: &RawStr, age: usize, gender: &RawStr) -> String {
    format!("{} : {} : {}", name, age, gender)
}

#[get("/<name>/<age>/<gender>", rank = 2)]
pub fn user_ssb(name: String, age: String, gender:bool) -> String {
    if gender == true {
        format!("{} : {} : Male", name, age)
    } else {
        format!("{} : {} : Female", name, age)
    }
}