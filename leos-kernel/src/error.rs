#[derive(Debug)]
pub enum KError {
    NoFB,
    FBCreated,
    OutOfRange,
}

pub type KResult<T> = Result<T, KError>;
