use std::fmt::Display;

pub struct Hint {
    pub title: String,
    pub before_text: String,
    pub after_text: String,
    pub items: Vec<String>,
    max_width: usize,
}

impl Display for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n{}{}\n",
            self.title,
            if self.before_text.is_empty() {
                "".into()
            } else {
                format!("{}\n\n", self.before_text)
            },
            self.items.join("\n")
        )
    }
}

#[derive(Debug)]
pub struct HintBuilder {
    title: String,
    before_text: String,
    after_text: String,
    items: Vec<String>,
    max_width: usize,
}

impl Default for HintBuilder {
    fn default() -> Self {
        Self {
            max_width: 80,
            title: Default::default(),
            before_text: Default::default(),
            after_text: Default::default(),
            items: Default::default(),
        }
    }
}

impl Hint {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> HintBuilder {
        HintBuilder::default()
    }
}

impl HintBuilder {
    pub fn with_title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn with_before_text<T>(&mut self, before_text: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.before_text = before_text.into();
        self
    }

    pub fn with_after_text<T>(&mut self, after_text: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.after_text = after_text.into();
        self
    }

    pub fn with_items(&mut self, items: Vec<impl Display>) -> &mut Self {
        self.items = items.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(&self) -> Hint {
        Hint {
            before_text: self.before_text.clone(),
            after_text: self.after_text.clone(),
            title: self.title.clone(),
            items: self.items.clone(),
            max_width: self.max_width,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hint_display() {
        let hint = Hint::builder().with_title("This is a test title").build();
        println!("{hint}")
    }

    #[test]
    fn hint_display_too_long() {
        let hint = Hint::builder()
            .with_title("This is a super duper long test title which overflows the max width")
            .with_before_text("This appears after the title and should be indented with 2 spaces")
            .build();

        println!("{hint}")
    }
}
