const API_AUTH: &str = "https://edge.microsoft.com/translate/auth";
const API_TRANSLATE: &str = "https://api.cognitive.microsofttranslator.com/translate";
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36 Edg/133.0.0.0";

use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextItem {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationItem {
    pub detected_language: DetectedLanguage,
    pub translations: Vec<Translation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedLanguage {
    pub language: String,
    pub score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    pub text: String,
    pub to: String,
}

pub struct Token {
    token: String,
    token_expires_at: u64,
}

impl Token {
    pub fn from_jwt(jwt: impl AsRef<str>) -> Self {
        let payload = serde_json::from_slice::<serde_json::Value>(&URL_SAFE.decode(jwt.as_ref().split('.').nth(1).unwrap_or_default()).unwrap_or_default()).unwrap_or_default();
        let token_expires_at = payload.get("exp").and_then(|x| x.as_u64()).unwrap_or_else(|| SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 600);
        Self {
            token: jwt.as_ref().to_string(),
            token_expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        self.token_expires_at < timestamp
    }
}

pub struct Client {
    token: Option<Token>,
    user_agent: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            token: None,
            user_agent: DEFAULT_USER_AGENT.to_string(),
        }
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    pub fn fetch_new_token() -> Result<String, ureq::Error> {
        Ok(ureq::get(API_AUTH).call()?.into_body().read_to_string()?)
    }

    fn get_token(&mut self) -> Result<&str, ureq::Error> {
        if self.token.is_none() || self.token.as_ref().unwrap().is_expired() {
            let token = Self::fetch_new_token()?;
            self.token = Some(Token::from_jwt(&token));
        }
        Ok(self.token.as_ref().unwrap().token.as_str())
    }

    /// Translate automatically detected language to the target language
    pub fn translate_to(&mut self, text: impl AsRef<str>, to: impl AsRef<str>) -> Result<String, ureq::Error> {
        self.translate_from_to(text, "", to)
    }

    /// Translate from the source language to the target language
    pub fn translate_from_to(&mut self, text: impl AsRef<str>, from: impl AsRef<str>, to: impl AsRef<str>) -> Result<String, ureq::Error> {
        // https://learn.microsoft.com/azure/ai-services/translator/reference/v3-0-translate#optional-parameters
        let response = ureq::post(&format!("{}?api-version=3.0&to={}&from={}", API_TRANSLATE, to.as_ref(), from.as_ref()))
            .header("User-Agent", DEFAULT_USER_AGENT)
            .header("Authorization", format!("Bearer {}", self.get_token()?))
            .header("Content-Type", "application/json; charset=UTF-8")
            .send_json(&vec![TextItem { text: text.as_ref().to_string() }])?
            .into_body()
            .read_json::<Vec<TranslationItem>>()?;
        Ok(response.into_iter().next().unwrap_or(TranslationItem::default()).translations.into_iter().next().unwrap_or(Translation::default()).text)
    }
}
