use failure::Fail;

#[derive(Debug, Fail, Clone)]
pub enum WalletErrorKind {
    #[fail(display = "Success")]
    Success,
    #[fail(display = "The wallet with the specified config already exists: {}", msg)]
    AlreadyExists { msg: String },
    #[fail(display = "The specified wallet could not be delete: {}", msg)]
    CantDelete { msg: String },
    #[fail(display = "The specified wallet could not be found: {}", msg)]
    NotFound { msg: String }
}

impl_error!(WalletError, WalletErrorKind);
