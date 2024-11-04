#[cfg(feature = "library")]
use mantle_utilities::{MantleError, ErrorType};

pub struct ErrorUtil {}

#[cfg(feature = "library")]
impl ErrorUtil {

    pub fn not_found_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::NotFound, 
            description: "Not found".to_string() 
        }
    }
    
    pub fn server_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::ServerError, 
            description: message
        }
    }

    pub fn too_many_instances_error(message: String) -> MantleError {
        MantleError {
            error_type: ErrorType::TooManyInstancesError,
            description: message
        }
    }

    pub fn user_session_not_found_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::UserSessionNotFound, 
            description: "User session not found".to_string() 
        }
    }
    
    pub fn generic_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::GenericError, 
            description: "Generic error".to_string() 
        }
    }
    
    pub fn email_or_phone_number_missing_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::EmailOrPhoneNumberMissing, 
            description: "Need either email or phone number to send confirmation".to_string() 
        }
    }
    
    pub fn passwords_mismatch_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::PasswordsMismatch, 
            description: "Passwords do not match".to_string() 
        }
    }
    
    pub fn path_empty_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::PathEmpty, 
            description: "Path is empty, not able to create a file directory".to_string() 
        }
    }
    
    pub fn malformed_or_incorrect_path(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::MalformedOrIncorrectPath, 
            description: message.to_string() 
        }
    }
    
    pub fn refresh_token_failed() -> MantleError {
        MantleError { 
            error_type: ErrorType::RefreshTokenFailed, 
            description: "Refresh token for API call failed".to_string() 
        }
    }
    
    pub fn invalid_method() -> MantleError {
        MantleError { 
            error_type: ErrorType::InvalidMethod, 
            description: "Invalid method".to_string() 
        }
    }
    
    pub fn disk_read_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::DiskReadError, 
            description: format!("Not able to read from disk  -> {:?}", &message) 
        }
    }
    
    pub fn disk_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::DiskError, 
            description: format!("Disk error  -> {:?}", &message) 
        }
    }
    
    pub fn datapoints_missing() -> MantleError {
        MantleError { 
            error_type: ErrorType::DatapointsMissing, 
            description: "Property has no datapoints".to_string()
        }
    }
    
    pub fn url_not_string() -> MantleError {
        MantleError { 
            error_type: ErrorType::URLNotString, 
            description: "Property value was not a string".to_string()
        }
    }
    
    pub fn local_file_name_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::LocalFileNameError, 
            description: "Error getting file name for local file".to_string()
        }
    }
    
    pub fn cached_directory_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::CachedDirectoryError, 
            description: "Could not get cache directory to save file".to_string()
        }
    }
    
    pub fn property_not_found() -> MantleError {
        MantleError { 
            error_type: ErrorType::PropertyNotFound, 
            description: "No property returned".to_string()
        }
    }
    
    pub fn url_not_found() -> MantleError {
        MantleError { 
            error_type: ErrorType::URLNotFound, 
            description: "No url was found for value".to_string()
        }
    }
    
    pub fn property_not_message_type() -> MantleError {
        MantleError { 
            error_type: ErrorType::PropertyNotMessageType, 
            description: "Property is not a message type".to_string()
        }
    }
    
    pub fn property_not_string_type() -> MantleError {
        MantleError { 
            error_type: ErrorType::PropertyNotStringType, 
            description: "Property value was not a string".to_string()
        }
    }
    
    pub fn invalid_format() -> MantleError {
        MantleError { 
            error_type: ErrorType::InvalidFormat, 
            description: "Value is not correct format".to_string()
        }
    }
    
    pub fn create_datapoint_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::CreateDatapointError, 
            description: format!("Could not create new datapoint  -> {:?}", &message) 
        }
    }
    
    pub fn save_datapoint_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::SaveDatapointError, 
            description: format!("Could not create datapoint to save file  -> {:?}", &message) 
        }
    }
    
    pub fn send_file_stream(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::SendFileStream, 
            description: format!("Error sending file stream data  -> {:?}", &message) 
        }
    }
    
    pub fn file_mark_complete_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::FileMarkCompleteError, 
            description: format!("Error marking file as complete  -> {:?}", &message) 
        }
    }
    
    pub fn file_datapoint_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::FileDatapointError, 
            description: format!("Error getting message datapoint  -> {:?}", &message) 
        }
    }
    
    pub fn message_datapoint_error(message: String) -> MantleError {
        MantleError { 
            error_type: ErrorType::MessageDatapointError, 
            description: format!("Error getting message datapoint  -> {:?}", &message) 
        }
    }
    
    pub fn parent_directory_mismatch() -> MantleError {
        MantleError { 
            error_type: ErrorType::ParentDirectoryMismatch, 
            description: "Parent directory does not exist".to_string()
        }
    }
    
    pub fn parent_directory_missing() -> MantleError {
        MantleError { 
            error_type: ErrorType::ParentDirectoryMissing, 
            description: "Parent directory does not exist".to_string()
        }
    }
    
    pub fn child_directory_missing() -> MantleError {
        MantleError { 
            error_type: ErrorType::ChildDirectoryMissing, 
            description: "Child directory does not exist".to_string()
        }
    }
    
    pub fn cache_key_missing() -> MantleError {
        MantleError { 
            error_type: ErrorType::CacheKeyMissing, 
            description: "A key needs to provided to retrieve cache".to_string()
        }
    }
    
    pub fn int_parse_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::IntParseError, 
            description: "Not able to parse i64 to i32".to_string()
        }
    }
    
    pub fn double_parse_error() -> MantleError {
        MantleError { 
            error_type: ErrorType::DoubleParseError, 
            description: "Not able to retrieve double".to_string()
        }
    }
    pub fn login_error(payload: String) -> MantleError {
        let error_type = if payload.contains("Invalid email or password") {
            ErrorType::InvalidEmailOrPassword
        } else if payload.contains("Your account is locked") {
            ErrorType::AccountLocked
        } else {
            ErrorType::ServerError
        };
        MantleError { 
            error_type,
            description: payload
        }
    }
}
