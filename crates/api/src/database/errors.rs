#[derive(Debug)]
pub enum DatabaseError {
    UnableToReadRoads,
    UnableToCreateRoad,
    UnableToDeleteRoad,
}

impl ToString for DatabaseError {
    fn to_string(&self) -> String {
        match self {
            DatabaseError::UnableToCreateRoad => "Could not create road",
            DatabaseError::UnableToReadRoads => "Could not read the roads",
            DatabaseError::UnableToDeleteRoad => "Could not delete road",
        }
        .into()
    }
}
