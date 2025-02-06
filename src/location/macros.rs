#[macro_export]
macro_rules! location {
    ($id:expr, $text:expr) => {
        Location {
            id: LocationID::from($id),
            text: String::from($text),
        }
    };
}
