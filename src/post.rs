use std::fs::{self, File};
use std::path::{Path, PathBuf};

use handlebars::{Handlebars, RenderError};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Post {
    title: String,
    pub created_date_time: String,
    parent_dir: PathBuf,
    pub dir: String,
    pub file_name: String,
    contents: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Header {
    pub title: String,
    pub date_time: String,
    tags: String,
}

impl Header {
    pub fn generate_url(&self) -> String {
        let date = self.date_time.split_whitespace().next().unwrap();
        let v: Vec<&str> = date.split('-').collect();
        format!("{}/{}/{}", v[0], v[1], v[2])
    }

    pub fn build_tags(&self) -> Vec<String> {
        self.tags
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}

impl Post {
    pub fn new(parent_dir: &Path, header: &Header, file_name: String, contents: String) -> Post {
        Post {
            title: header.title.clone(),
            created_date_time: header.date_time.clone(),
            parent_dir: parent_dir.to_path_buf(),
            dir: header.generate_url(),
            file_name,
            contents,
            tags: header.build_tags(),
        }
    }

    pub fn render(&self, hbs: &Handlebars) -> Result<(), RenderError> {
        let file_dir = self.parent_dir.join(&self.dir);
        fs::create_dir_all(&file_dir).unwrap();

        let mut f = file_dir.join(&self.file_name);
        f.set_extension("html");
        let file = File::create(f).unwrap();
        hbs.render_to_write(
            "post",
            &json!({
                "parent": "layout",
                "post": self}),
            file,
        )?;

        Ok(())
    }
}

#[test]
fn test_new_header() {
    let header = Header {
        title: String::from("test"),
        date_time: String::from("aa"),
        tags: String::from("hello world"),
    };
    let v1: Vec<&str> = header.tags.split_whitespace().collect();
    assert_eq!("hello", v1[0]);
    assert_eq!("world", v1[1]);
}

#[test]
fn test_new_post() {
    let header = Header {
        title: String::from("test"),
        date_time: String::from("2019-2-25"),
        tags: String::from("hello world"),
    };
    let post = Post::new(
        Path::new("a"),
        &header,
        String::from("ssssss"),
        String::from("sss"),
    );
    assert_eq!("test", post.title);
    assert_eq!("world", post.tags[1]);
}
