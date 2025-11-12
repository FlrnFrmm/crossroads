use std::path::PathBuf;

pub(super) fn is_valid_port(value: &u16, _: &()) -> garde::Result {
    match value {
        80 => Ok(()),
        443 => Ok(()),
        v if *v > 1023 => Ok(()),
        _ => {
            let error_message = format!(
                "Invalid port value of {}. Value has to be 80, 443 or greater than 1023",
                value
            );
            Err(garde::Error::new(error_message))
        }
    }
}

pub(super) fn is_valid_path(path: &PathBuf, _: &()) -> garde::Result {
    if !path.is_file() {
        let error_message = format!("{:?} is not a valid path/file", path.as_os_str());
        return Err(garde::Error::new(error_message));
    }
    match path.try_exists() {
        Ok(_) => Ok(()),
        Err(error) => {
            let error_message = format!(
                "\"{:?}\" is not a valid path: {}",
                path.as_os_str(),
                error.to_string()
            );
            Err(garde::Error::new(error_message))
        }
    }
}
