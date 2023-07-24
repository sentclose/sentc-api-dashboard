use alloc::string::String;

use sentc_crypto_utils::error::SdkUtilError;
use serde::Serialize;

pub fn to_string<T: ?Sized + Serialize>(obj: &T) -> Result<String, SdkUtilError>
{
	serde_json::to_string(obj).map_err(|_e| SdkUtilError::JsonToStringFailed)
}
