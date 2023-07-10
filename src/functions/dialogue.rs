use dialoguer::{theme::ColorfulTheme, FuzzySelect};
pub fn create_dialogue(options: Vec<&str>, question: &str) -> usize {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    selection
}
