use chrono::NaiveDate;

/// A single todo item following the todo.txt format.
#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub done: bool,
    pub priority: Option<char>,
    pub completion_date: Option<NaiveDate>,
    pub creation_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub description: String,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
}

impl Todo {
    pub fn new(description: impl Into<String>, priority: Option<char>) -> Self {
        let description = description.into();
        let contexts = parse_tokens(&description, '@');
        let projects = parse_tokens(&description, '+');
        let due_date = parse_due_date(&description);
        Self {
            done: false,
            priority,
            completion_date: None,
            creation_date: Some(chrono::Local::now().date_naive()),
            due_date,
            description,
            contexts,
            projects,
        }
    }

    /// Parse a todo.txt line into a Todo.
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let mut rest = line;
        let mut done = false;
        let mut completion_date: Option<NaiveDate> = None;
        let mut creation_date: Option<NaiveDate> = None;
        let mut priority: Option<char> = None;

        // Check for completion marker
        if let Some(after_x) = rest.strip_prefix("x ") {
            done = true;
            rest = after_x;
            // Try to parse completion date
            if let Some((date_str, remainder)) = rest.split_once(' ')
                && let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            {
                completion_date = Some(date);
                rest = remainder;
            }
        }

        // Check for priority: "(A) "
        if rest.starts_with('(')
            && rest.len() >= 4
            && rest.as_bytes()[2] == b')'
            && rest.as_bytes()[3] == b' '
        {
            let c = rest.as_bytes()[1] as char;
            if c.is_ascii_uppercase() {
                priority = Some(c);
                rest = &rest[4..];
            }
        }

        // Try to parse creation date
        if let Some((date_str, remainder)) = rest.split_once(' ')
            && let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        {
            creation_date = Some(date);
            rest = remainder;
        }

        let description = rest.to_string();
        let contexts = parse_tokens(&description, '@');
        let projects = parse_tokens(&description, '+');
        let due_date = parse_due_date(&description);

        Some(Self {
            done,
            priority,
            completion_date,
            creation_date,
            due_date,
            description,
            contexts,
            projects,
        })
    }

    /// Serialize to a todo.txt line.
    pub fn to_line(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        if self.done {
            parts.push("x".to_string());
            if let Some(date) = self.completion_date {
                parts.push(date.format("%Y-%m-%d").to_string());
            }
        }

        if let Some(p) = self.priority
            && !self.done
        {
            parts.push(format!("({p})"));
        }

        if let Some(date) = self.creation_date {
            parts.push(date.format("%Y-%m-%d").to_string());
        }

        parts.push(self.description.clone());
        parts.join(" ")
    }

    /// Priority sort key: A=0, B=1, …, E=4, F/None = 26 (backlog).
    pub fn priority_order(&self) -> u8 {
        match self.priority {
            Some(c @ 'A'..='E') => c as u8 - b'A',
            _ => 26,
        }
    }
}

fn parse_tokens(description: &str, prefix: char) -> Vec<String> {
    description
        .split_whitespace()
        .filter_map(|word| {
            word.strip_prefix(prefix)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
        .collect()
}

fn parse_due_date(description: &str) -> Option<NaiveDate> {
    description.split_whitespace().find_map(|word| {
        word.strip_prefix("due:")
            .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_todo() {
        let todo = Todo::parse("(A) Buy milk @errands +shopping").unwrap();
        assert_eq!(todo.priority, Some('A'));
        assert_eq!(todo.description, "Buy milk @errands +shopping");
        assert_eq!(todo.contexts, vec!["errands"]);
        assert_eq!(todo.projects, vec!["shopping"]);
        assert!(!todo.done);
    }

    #[test]
    fn parses_completed_todo() {
        let todo = Todo::parse("x 2026-03-08 2026-03-01 Done task @work").unwrap();
        assert!(todo.done);
        assert_eq!(
            todo.completion_date,
            Some(NaiveDate::from_ymd_opt(2026, 3, 8).unwrap())
        );
    }

    #[test]
    fn parses_no_priority() {
        let todo = Todo::parse("Simple task").unwrap();
        assert_eq!(todo.priority, None);
        assert_eq!(todo.priority_order(), 26);
    }

    #[test]
    fn serializes_roundtrip() {
        let line = "(B) 2026-03-01 Fix bug @work +devtool";
        let todo = Todo::parse(line).unwrap();
        assert_eq!(todo.to_line(), line);
    }

    #[test]
    fn parses_due_date() {
        let todo = Todo::parse("(A) Fix bug due:2026-03-15 @work").unwrap();
        assert_eq!(
            todo.due_date,
            Some(NaiveDate::from_ymd_opt(2026, 3, 15).unwrap())
        );
    }

    #[test]
    fn new_extracts_context_from_description() {
        let todo = Todo::new("Write report @work +devtool", Some('B'));
        assert_eq!(todo.contexts, vec!["work"]);
        assert_eq!(todo.projects, vec!["devtool"]);
    }
}
