extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expect a target to match against");
    use pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel.delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error when deleting posts");
    
    println!("Deleted {} posts", num_deleted);
}