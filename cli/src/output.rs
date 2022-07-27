use structopt::StructOpt;

#[derive(StructOpt, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    #[structopt(about = "Shows responses using the ESC cli's custom output. Deprecated.")]
    Cli,
    #[structopt(
        about = "Shows responses using a JSON form of the ESC cli's custom output. Deprecated."
    )]
    CliJson,
    #[structopt(about = "Shows response bodies exactly as they appear in the API")]
    Api,
    #[structopt(
        about = "Show all request / response traffic. Hides token, but may show sensitive data in the request body if any. Overrides all other output options"
    )]
    ApiVerbose,
}

static OUTPUT_FORMAT_HELP: &str = r#"
Output format options:
    api - Shows response bodies exactly as they appear in the API.
    cli - Shows responses using the ESC cli's custom output format. Deprecated. 
    cli-json - Shows responses using the ESC cli's custom output format, but serialized back into JSON. Deprecated.
"#;

impl OutputFormat {
    pub fn is_v1(&self) -> bool {
        match self {
            Self::Cli => true,
            Self::CliJson => true,
            Self::Api => false,
            Self::ApiVerbose => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Cli => "cli",
            Self::CliJson => "cli-json",
            Self::Api => "api",
            Self::ApiVerbose => "api-verbose",
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(Self::Cli),
            "cli-json" => Ok(Self::CliJson),
            "api" => Ok(Self::Api),
            "api-verbose" => Ok(Self::ApiVerbose),
            _ => {
                eprintln!("Error parsing `fmt` option: unknown output format type: {s}\n{OUTPUT_FORMAT_HELP}");
                Err(format!("unknown output format type: {s}"))
            }
        }
    }
}
