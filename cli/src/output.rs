use structopt::StructOpt;

#[derive(StructOpt, Deserialize, Serialize, Clone, Debug)]
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
            _ => Err(format!("unknown output format type: {s}")),
        }
    }
}
