use std::fs::File;
use std::io::{self, Read};

#[derive(Clone, Debug)]
pub struct TagRecord {
    pub name: String,
    pub tag_type: String,
    pub post_count: i32,
    pub aliases: Vec<String>,
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn read_CSV(filename: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let mut reader = csv::Reader::from_path(filename)?;
    let mut records: Vec<Vec<String>> = Vec::new();
    for result in reader.records() {
        let record = result?;
        records.push(record.deserialize(None).unwrap());
    }
    Ok(records)
}

pub fn parse_tags(tags: Vec<Vec<String>>) -> Vec<TagRecord> {
    let mut tag_records: Vec<TagRecord> = Vec::new();
    for tag in tags {
        let tag_record = TagRecord {
            name: tag[0].clone(),
            tag_type: tag[1].clone(),
            post_count: tag[2].parse().unwrap(),
            aliases: tag[3..].to_vec(),
        };
        tag_records.push(tag_record);
    }
    tag_records
}
