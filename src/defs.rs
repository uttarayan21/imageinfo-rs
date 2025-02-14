#[derive(PartialEq)]
#[derive(Debug)]
pub struct ImageSize {
    pub width: i64,
    pub height: i64,
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{{width: {}, height: {}}}", self.width, self.height))
    }
}

#[derive(Debug)]
pub enum ImageInfoError {
    UnrecognizedFormat,
    IoError(std::io::Error),
}

impl std::error::Error for ImageInfoError {}

impl std::fmt::Display for ImageInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnrecognizedFormat => f.write_str("unrecognized image format"),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for ImageInfoError {
    fn from(err: std::io::Error) -> ImageInfoError {
        ImageInfoError::IoError(err)
    }
}

pub type ImageInfoResult<T> = std::result::Result<T, ImageInfoError>;
