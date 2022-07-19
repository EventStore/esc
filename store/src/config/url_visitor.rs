use super::invalid_url::InvalidUrl;
use std::fmt::Formatter;

pub fn parse_url(str: &str) -> Result<url::Url, Box<dyn std::error::Error>> {
    let url = url::Url::parse(str)?;

    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(InvalidUrl {}.into());
    }

    if url.host().is_none() {
        return Err(InvalidUrl {}.into());
    }

    Ok(url)
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(UrlVisitor {})
}

pub fn serialize_url<S>(url: &Option<url::Url>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(url) = url {
        return serializer.serialize_some(url.as_str());
    }

    serializer.serialize_none()
}

pub struct UrlVisitor {}

impl<'a> serde::de::Visitor<'a> for UrlVisitor {
    type Value = Option<url::Url>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a valid URL")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match parse_url(value) {
            Ok(url) => Ok(Some(url)),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}
