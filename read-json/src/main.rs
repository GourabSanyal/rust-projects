use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn main() {
    let json = r#"
    {
        "article" : "working with json in rust",
        "author" : "Hirimba",
        "paragraph" : [
            {
                "name" :  "first sentance"

            },
            {
                "name" : "paragraph body"
            },
            {
                "name":"this is the third sentacne"
            }
            ]
        }"#;
        let parsed : Article = read_json_types(json);
        println!("/n/n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_types(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}