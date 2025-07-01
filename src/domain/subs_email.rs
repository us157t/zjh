use validator::validate_email;


#[derive(Debug)]
pub struct SubsEmail(String);

impl SubsEmail {
	pub fn parse(s: String) -> Result<SubsEmail, String> {
		if validate_email(&s) {
			Ok(Self(s))
		} else {
			Err(format!("{} is not a valid subs email.", s))
		}
	}
}

impl AsRef<str> for SubsEmail {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use fake::faker::internet::en::SafeEmail;
	use fake::Fake;
	use super::SubsEmail;
	use claim::assert_err;

	#[test]
	fn empty1234() {
		let email = SafeEmail().fake();
		claim::assert_ok!(SubsEmail::parse(email));
	}


}

