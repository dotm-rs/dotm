use std::fmt::Display;

pub struct Hint {
    pub title: MaxWidthString,
    pub before_text: MaxWidthString,
    pub after_text: MaxWidthString,
    pub items: Vec<String>,
    max_width: usize,
}

impl Display for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\n{}", self.title, self.before_text)
    }
}

pub struct MaxWidthString {
    max_width: usize,
    padding: usize,
    output: String,
    line: String,
}

impl MaxWidthString {
    pub fn new(content: String, max_width: usize, padding: usize) -> Self {
        let mut output = String::from(" ".repeat(padding));
        let mut line = String::new();

        let mut words: Vec<&str> = content.split(' ').collect();
        words.reverse();

        while let Some(word) = words.pop() {
            if word.len() + line.len() + 1 > max_width - padding {
                output.push_str(&line);
                output.push('\n');
                output.push_str(" ".repeat(padding).as_str());

                line.clear();
                words.push(word);

                continue;
            }

            line.push_str(word);
            line.push(' ');
        }

        Self {
            max_width,
            padding,
            output,
            line,
        }
    }

    pub fn push_string(&mut self, content: String) {
        let mut words: Vec<&str> = content.split(' ').collect();
        words.reverse();

        while let Some(word) = words.pop() {
            if word.len() + self.line.len() + 1 > self.max_width - self.padding {
                self.output.push_str(&self.line);
                self.output.push('\n');
                self.output.push_str(" ".repeat(self.padding).as_str());

                self.line.clear();
                words.push(word);

                continue;
            }

            self.line.push_str(word);
            self.line.push(' ');
        }
    }

    fn display(&self) -> String {
        let mut output = self.output.to_owned();
        output.push_str(&self.line);

        output
    }
}

impl Display for MaxWidthString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.display();
        write!(f, "{output}")
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

    pub fn with_items(&mut self, items: Vec<String>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn with_max_width(&mut self, max_width: usize) -> &mut Self {
        self.max_width = max_width;
        self
    }

    pub fn build(&self) -> Hint {
        Hint {
            before_text: MaxWidthString::new(self.before_text.clone(), self.max_width, 2),
            after_text: MaxWidthString::new(self.after_text.clone(), self.max_width, 2),
            title: MaxWidthString::new(self.title.clone(), self.max_width, 0),
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
            .with_max_width(50)
            .build();

        println!("{hint}")
    }
}
