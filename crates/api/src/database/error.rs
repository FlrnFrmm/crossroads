#[derive(Debug)]
pub enum Error {
    UnableToReadRoads,
    UnableToCreateRoad,
    UnableToDeleteRoad,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnableToCreateRoad => "Could not create road",
            Error::UnableToReadRoads => "Could not read the roads",
            Error::UnableToDeleteRoad => "Could not delete road",
        }
        .into()
    }
}
