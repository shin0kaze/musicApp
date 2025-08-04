#[derive(Debug)]
struct Track {
    id: i32,
    title: Option<String>,
    album: Option<String>,
    duration: Duration,
    remixOf: Option<i32>,
    createdAt: SystemTime,
    addedAt: SystemTime,
    updatedAt: SystemTime,
    authors: Vec<i32>,
}

#[derive(Debug)]
struct Author {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct FileSource {
    id: i32,
    srcType: SourceType,
    path: String,
}

#[derive(Debug)]
struct SourceType {
    id: i16,
    priority: SourceType,
    description: String,
}

#[derive(Debug)]
struct Tag {
    id: i32,
    name: String,
    desc: String,
    fromTag: Vec<i32>,
}



