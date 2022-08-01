use std::collections::HashMap;

/// Represents a problem reported from the API
#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn problem_details_display() {
        let mut fields = HashMap::new();
        fields.insert("field1".to_string(), "value1".to_string());
        fields.insert("field2".to_string(), "value2".to_string());
        let pd = ProblemDetails {
            detail: "Details Here".to_string(),
            fields,
            instance: "Instance".to_string(),
            status: "Status".to_string(),
            title: "Title".to_string(),
            _type: "Type".to_string(),
        };
        let expected = r#"\{"detail": "Details Here", "fields": (\{"field1": "value1", "field2": "value2"\}|\{"field2": "value2", "field1": "value1"\}), "instance": "Instance", "status": "Status", "title": "Title", "type": "Type"\}"#;
        let actual = format!("{}", pd);
        let expected_re = Regex::new(expected).unwrap();
        assert!(
            expected_re.is_match(&actual),
            "Error: expected {}, actual {}",
            expected,
            actual
        );
    }
}
