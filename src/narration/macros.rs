#[macro_export]
macro_rules! narration {
    ($id:expr, $text:expr) => {
        Narration {
            id: NarrationID::from($id),
            text: String::from($text),
        }
    };
}
