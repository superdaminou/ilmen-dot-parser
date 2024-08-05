use thiserror::Error;


#[derive(Error, Debug)]
pub enum ParsingError
{
    #[error("An error occured: {0}")]
    DefaultError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
