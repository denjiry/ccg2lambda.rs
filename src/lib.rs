#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;

#[test]
fn test() {
    func()
}

#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}

pub fn func() {
    let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
        </Project>
    "##;
    let project: Project = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", project);
}
