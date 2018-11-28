#![no_std]

#[derive(Debug, PartialEq)]
pub enum SGFParsingError {
    MissingTreeError,
    TreeWithNoNodesError,
}

enum ParseState {
    MissingTree,
    NewTree,
    Error(SGFParsingError),
}

fn process_char(c: char, state: &ParseState) -> ParseState {
    match state {
        ParseState::MissingTree => {
            if c == '(' {
                ParseState::NewTree
            } else {
                ParseState::Error(SGFParsingError::MissingTreeError)
            }
        }

        ParseState::NewTree => {
            if c == ')' {
                ParseState::Error(SGFParsingError::TreeWithNoNodesError)
            } else {
                ParseState::NewTree
            }
        }

        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Result<(), SGFParsingError> {
    let mut state = ParseState::MissingTree;

    for c in input.chars() {
        state = process_char(c, &state);

        if let ParseState::Error(err) = state {
            return Err(err);
        }
    }

    match state {
        ParseState::MissingTree => Err(SGFParsingError::MissingTreeError),
        _ => Ok(()),
    }
}
