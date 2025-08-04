
const create_tbl_track : &str = "
create table track (
    id        integer primary key,
    title     text,
    album     text,
    duration  real not null,
    remixOf   integer not null,
    createdAt text,
    addedAt   text,
    updatedAt text,
    authors   blob,
)";

const create_tbl_author : &str = "
create table author (
    id   integer primary key,
    name text,
)";

const create_tbl_file_source : &str = "
create table fileSource (
    id     integer primary key,
    typeId integer not null,
    path   text,
)";

const create_tbl_source_type : &str = "
create table sourceType (
    id         integer primary key,
    dwnMethods text,
    priority   text,
    desc       text,
)";

const create_tbl_tag : &str = "
create table tag (
    id      integer primary key,
    name    text,
    desc    text,
    fromTag integer,
)";

const create_tbl_m_2_m : &str = "
create table {name1}_{name2} (
    trackId      integer primary key,
    author_id    text,
    PRIMARY KEY (student_id, course_id)
) without rowid";

pub const test : &str = "hi {subj}!";

 //CREATE INDEX idx_enrollments_course_id ON Enrollments(course_id);

//написать интерполяцию

//#[path = "routes/login_route.rs"]