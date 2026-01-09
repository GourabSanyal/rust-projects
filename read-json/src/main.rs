use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)];
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)];
struct Article {
    article: String,
    auther: String,
    paragrapg: Vec<Paragraph>
}
