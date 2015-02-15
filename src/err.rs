#![feature(core)]
use std::fmt::Formatter;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;

#[derive(Copy, PartialEq, Eq, Debug)]
pub enum DDSError {
    Ok = 0, 
    Error = 1,
    BadParameter = 2,
    Unsupported = 3,
    AlreadyDeleted = 4,
    OutOfResources = 5,
    NotEnabled = 6,
    ImmutablePolicy = 7,
    InconsistentPolicy = 8,
    PreconditionNotMet = 9,
    Timeout = 10,
    IllegalOperation = 11,
    NoData = 12,
    AlreadyExisted = 13,
}

impl Error for DDSError {
    fn description(&self) -> &str {
        match *self {
            DDSError::Ok => 
                "Successful return",
            DDSError::Error => 
                "Generic, unspecified error",
            DDSError::BadParameter => 
                "Illegal parameter value",
            DDSError::Unsupported => 
                "Unsupported operation",
            DDSError::AlreadyDeleted => 
                "The object has already been deleted",
            DDSError::OutOfResources => 
                "Service ran out of the resources needed to complete the operation",
            DDSError::NotEnabled => 
                "Operation invoked on an Entity that is not yet enabled",
            DDSError::ImmutablePolicy => 
                "Application attempted to modify an immutable QosPolicy",
            DDSError::InconsistentPolicy => 
                "Application specified a set of policies that are not consistent with each other",
            DDSError::PreconditionNotMet => 
                "A pre-condition for the operation was not met",
            DDSError::Timeout => 
                "The operation timed out",
            DDSError::IllegalOperation => 
                "An operation was invoked on an inappropriate object or at an inappropriate time",
            DDSError::NoData => 
                "the operation did not return any data but there is no inherent error", 
            DDSError::AlreadyExisted => 
                "Default",
        }
    }
}

impl Display for DDSError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.description().fmt(f)
    }
}