use std::{
    collections::HashSet,
    time::{Instant, SystemTime},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Query {
    generator: Generator,
    expression: Expression,
    fields: HashSet<FieldSpec>,
}

/// Fields requested by query
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FieldSpec {
    /// The filename, relative to the watched root
    Name,
    /// True if the file exists
    Exists,
    /// The “created clock”; the clock value when we first observed the file,
    /// or the clock value when it last switched from !exists to exists
    CClock,
    /// The “observed clock”; the clock value where we last observed some
    /// change in this file or its metadata
    OClock,
    /// Last inode change time, in a given resolution
    CTime(TimeRes),
    /// Modified time, in a given resolution
    MTime(TimeRes),
    /// File size in bytes
    Size,
    /// File or directory mode
    Mode,
    /// The owning user id
    Uid,
    /// The owning group id
    Gid,
    /// The inode number
    Ino,
    /// The device number
    Dev,
    /// The number of hard links
    NLink,
    /// Whether this entry is newer than the `since` generator criteria
    New,
    /// The file type
    Type,
    /// Symlink target
    SymlinkTarget,
    /// SHA-1 of file contents
    ContentSha1,
}

/// Time resolution to request for timestamps
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TimeRes {
    Sec,
    Msec,
    Usec,
    Nsec,
    Float,
}

/// Field results
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Field {
    /// The filename, relative to the watched root
    Name(String),
    /// True if the file exists
    Exists(bool),
    /// The “created clock”; the clock value when we first observed the file,
    /// or the clock value when it last switched from !exists to exists
    CClock(ClockSpec),
    /// The “observed clock”; the clock value where we last observed some
    /// change in this file or its metadata
    OClock(ClockSpec),
    /// Last inode change time
    CTime(Instant),
    /// Modified time, in seconds
    MTime(Instant),
    /// File size in bytes
    Size(u64),
    /// File or directory mode
    Mode(u32),
    /// The owning user id
    Uid(u32),
    /// The owning group id
    Gid(u32),
    /// The inode number
    Ino(u64),
    /// The device number
    Dev(u32),
    /// The number of hard links
    NLink(u32),
    /// Whether this entry is newer than the `since` generator criteria
    New(bool),
    /// The file type
    Type(FileType),
    /// Symlink target
    SymlinkTarget(String),
    /// SHA-1 of file contents
    ContentSha1([u8; 20]),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Clock(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClockSpec {
    Epoch(SystemTime),
    Clock(Clock),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Generator {
    All,
    Since { clock: ClockSpec, expr: Expression },
    Suffix(Vec<String>),
    Glob { globs: Vec<String>, fields: () },
    Path { paths: Vec<(String, Option<usize>)> },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    /// Match everything
    All,
    /// True when all expressions match
    AllOf(Vec<Expression>),
    /// True when any expression matches
    AnyOf(Vec<Expression>),
    /// True when dir name matches
    DirName {
        dir: String,
        case_sensitive: bool,
        depth: (Cmp, u32),
    },
    /// True when file is empty
    Empty,
    /// True when file exists
    Exists,
    /// Glob matching
    Match {
        glob: String,
        scope: MatchScope,
        case_sensitive: bool,
    },
    /// Name matching
    Name {
        names: Vec<String>,
        scope: MatchScope,
        case_sensitive: bool,
    },
    /// True when expression is false
    Not(Box<Expression>),
    /// Match name against regex
    Pcre {
        re: String,
        scope: MatchScope,
        case_sensitive: bool,
    },
    /// Match since some point in time
    Since(SinceSpec),
    /// True when file size matches comparison
    Size(Cmp, u64),
    /// True when file has one of the listed suffixes
    Suffix(Vec<String>),
    /// True when file type matches
    Type(FileType),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MatchScope {
    Basename,
    Wholename,
}

impl Default for MatchScope {
    fn default() -> Self {
        MatchScope::Basename
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MatchOptions {
    NoEscape,
    IncludeDotFiles,
}

/// Comparisons
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Cmp {
    /// Equals
    Eq,
    /// Not equal
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
}

/// File types
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FileType {
    /// Block device special
    Block,
    /// Character device special
    Char,
    /// Directory
    Dir,
    /// Regular file
    Regular,
    /// Named Pipe (FIFO) file
    NamedPipe,
    /// Symbolic link
    Symlink,
    /// Unix Domain Socket
    Socket,
    /// Solaris Door
    Door,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SinceSpec {
    MTime(SystemTime),
    CTime(SystemTime),
    OClock(ClockSpec),
    CClock(ClockSpec),
}
