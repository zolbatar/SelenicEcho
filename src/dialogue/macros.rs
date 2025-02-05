#[macro_export]
macro_rules! dialogue {
    ($id:expr, $speaker:expr, $text:expr, [ $(($opt_text:expr, $next_id:expr)),* ]) => {
        DialogueNode {
            id: String::from($id),
            speaker: String::from($speaker),
            text: String::from($text),
            options: vec![
                $(DialogueOption {
                    text: String::from($opt_text),
                    next: String::from($next_id),
                }),*
            ],
        }
    };
}
