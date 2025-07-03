use crate::domain::SubsEmail;
use reqwest::Client;

#[derive(Clone)]
pub struct EmailCli {
    http_client: Client,
    base_url: String,
    sender: SubsEmail,
}

impl EmailCli {
    pub fn new(base_url: String, sender: SubsEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        rec: SubsEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubsEmail;
    use crate::email_client::EmailCli;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    #[tokio::test]
    async fn send_email() {
        let mock_s = MockServer::start().await;
        let sender = SubsEmail::parse(SafeEmail().fake()).unwrap();
        let email_cli = EmailCli::new(mock_s.uri(), sender);

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_s)
            .await;

        let subs = SubsEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_cli
            .send_email(subs, &subject, &content, &content)
            .await;
    }
}
