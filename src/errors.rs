use std::{error::Error, fmt::Display};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * LIBRARY-SPECIFIC ISNSTANCE OF RESULT                                              *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub type NexusArtResult<OkType> = Result<OkType, NexusArtError>;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ERRORS                                                                            *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Debug)]
pub struct NexusArtError {
    message: String,
}

// Implementation of NexusArtError
impl NexusArtError {
    pub fn new<StringType>(function_path: &str, message: StringType) -> Self
    where
        StringType: Into<String>
    {
        NexusArtError{ message: format!("{}. {}", function_path, message.into()) }
    }
}

// Implementation of Display
impl Display for NexusArtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("NexusArtError: {}", self.message).as_str())
    }
}

// Implementation of Error
impl Error for NexusArtError {}
