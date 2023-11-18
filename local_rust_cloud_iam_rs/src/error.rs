use actix_http::StatusCode;

#[derive(Debug, Clone)]
pub enum IamErrorKind {
    InvalidInputException,
    LimitExceededException,
    EntityAlreadyExistsException,
    MalformedPolicyDocumentException,
    ConcurrentModificationException,
    ServiceFailureException,
}

#[derive(Debug, Clone)]
pub struct IamError {
    pub status: StatusCode,
    pub kind: IamErrorKind,
    pub msg: String,
}

impl From<IamError> for String {
    fn from(value: IamError) -> Self {
        todo!()
    }
}