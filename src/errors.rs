use std::{error::Error, fmt::Display};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * LIBRARY-SPECIFIC ISNSTANCE OF RESULT                                              *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub type CrabNetsResult<OkType> = Result<OkType, CrabNetsError>;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ERRORS                                                                            *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Debug)]
pub struct CrabNetsError {
    message: String,
}

// Implementation of CrabNetsError
impl CrabNetsError {
    pub fn new<StringType>(function_path: &str, message: StringType) -> Self
    where
        StringType: Into<String>
    {
        CrabNetsError{ message: format!("{}. {}", function_path, message.into()) }
    }
}

// Implementation of Display
impl Display for CrabNetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("CrabNetsError: {}", self.message).as_str())
    }
}

// Implementation of Error
impl Error for CrabNetsError {}
