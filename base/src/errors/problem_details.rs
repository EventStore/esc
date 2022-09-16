use std::collections::HashMap;

/// Represents a problem reported from the API
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub fields: Option<HashMap<String, String>>,
    pub instance: String,
    pub status: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl std::fmt::Display for ProblemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        write!(f, r#"{{"detail": {:?}, "fields": {{"#, self.detail)?;
        if let Some(fields) = &self.fields {
            for (count, (key, value)) in fields.iter().enumerate() {
                if count > 0 {
                    write!(f, ", ")?;
                }
                write!(f, r#"{:?}: {:?}"#, key, value)?;
            }
        }
        write!(
            f,
            r#"}}, "instance": {:?}, "status": {:?}, "title": {:?}, "type": {:?}}}"#,
            self.instance, self.status, self.title, self._type
        )
    }
}

impl std::fmt::Debug for ProblemDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "{}", self)
    }
}
