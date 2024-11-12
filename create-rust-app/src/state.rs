
pub enum State {
    Initialization,
    DirectoryStructure,
    ConfigurationFiles,
    Dependencies,
    CodeTemplates,
    Customization,
    Finalization,
    Done,
    Error(String),
}

impl State {
    pub fn next(&self) -> State {
        match self {
            State::Initialization => State::DirectoryStructure,
            State::DirectoryStructure => State::ConfigurationFiles,
            State::ConfigurationFiles => State::Dependencies,
            State::Dependencies => State::CodeTemplates,
            State::CodeTemplates => State::Customization,
            State::Customization => State::Finalization,
            State::Finalization => State::Done,
            State::Done => State::Done,
            State::Error(_) => State::Done,
        }
    }
}