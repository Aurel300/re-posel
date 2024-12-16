#[allow(dead_code)]
#[derive(Debug)]
pub enum DisError {
    TooShort,
    MagicMismatch,
    LengthMismatch,
    MalformedString,
    MalformedCode(String),
    CfgAnalysisFailed,
}
