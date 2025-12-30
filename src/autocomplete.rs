use inquire::{Autocomplete, autocompletion::Replacement};

#[derive(Clone)]
pub struct KeyCompleter {
    pub keys: Vec<String>,
}

impl Autocomplete for KeyCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let filtered_keys: Vec<String> = self
            .keys
            .iter()
            .filter(|k| k.starts_with(input))
            .cloned()
            .collect();

        Ok(filtered_keys)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        if highlighted_suggestion.is_some() {
            return Ok(Replacement::Some(highlighted_suggestion.unwrap()));
        }

        let completion = self.keys.iter().filter(|k| k.starts_with(input)).nth(0);
        match completion {
            Some(c) => Ok(Replacement::Some(c.clone())),
            None => Ok(Replacement::None),
        }
    }
}
