#[derive(Debug)]
pub enum Error {
    UnableToReadRoads,
    UnableToCreateRoad,
    UnableToUpdateRoad,
    UnableToDeleteRoad,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnableToCreateRoad => "Could not create road",
            Error::UnableToReadRoads => "Could not read the roads",
            Error::UnableToUpdateRoad => "Could not update road",
            Error::UnableToDeleteRoad => "Could not delete road",
        }
        .into()
    }
}
