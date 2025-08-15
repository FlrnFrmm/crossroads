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
