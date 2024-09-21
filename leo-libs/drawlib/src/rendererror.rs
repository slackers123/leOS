#[derive(Debug)]
pub enum RenderError {
    PathNotCached,
}

pub type RenderResult<T> = Result<T, RenderError>;
