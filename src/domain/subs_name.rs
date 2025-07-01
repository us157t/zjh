use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct SubsName(String);
impl SubsName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = s.chars().any(|g| forbidden_chars.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid subs name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubsName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}


pub struct NewSubs {
    pub email: String,
    pub name: SubsName,
}
