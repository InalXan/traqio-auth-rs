use csv::Writer;
use serde_json;
use std::error::Error;
use user_model::User;

// Model import
#[path = "../../src/data/user_model.rs"]
mod user_model;

pub async fn data_write() -> Result<(), Box<dyn Error>> {
    let all_user_data = vec![
        User {
            fullname: "John Doe".to_string(),
            username: "darkorjohn".to_string(),
            email: "john.doe@gmail.com".to_string(),
            number: "+99334343434".to_string(),
        },
        User {
            fullname: "Kate Swilson".to_string(),
            username: "swilsone".to_string(),
            email: "kateswilson@gmail.com".to_string(),
            number: "+99323423423".to_string(),
        },
    ];

    let mut wrt = Writer::from_path("./src/data/data.csv")?;

    for user in &all_user_data {
        wrt.serialize(user)?;
    }

    wrt.flush()?;
    Ok(())
}
