#[macro_export]
macro_rules! location {
    ($id:expr, $narration_id:expr) => {
        Location {
            id: LocationID::from($id),
            narration_id: NarrationID::from($narration_id),
        }
    };
}
