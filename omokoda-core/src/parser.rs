#[derive(Debug, PartialEq)]
pub struct MetadataPair {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Birth {
        name: String,
        metadata: Vec<MetadataPair>,
    },
    Think {
        prompt: String,
        private: bool,
    },
    Act {
        tool: String,
        params: String,
        sandbox: bool,
    },
    SlashCmd {
        command: String,
        arg: Option<String>,
    },
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

pub fn parse(input: &str) -> Result<Vec<Statement>, ParseError> {
    Err(ParseError(format!("not implemented: {input}")))
}
