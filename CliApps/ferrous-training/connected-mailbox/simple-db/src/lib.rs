#[derive(Debug, PartialEq)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyMessage,
    MissingPayload,
    IncompleteMessage,
    UnexpectedNewline,
    UnknownCommand,
    UnexpectedPayload,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let cmd = input.strip_suffix("\n").ok_or(Error::IncompleteMessage)?;

    if cmd.is_empty() {
        return Err(Error::EmptyMessage);
    }

    if let Some(payload) = cmd.strip_prefix("PUBLISH") {
        if payload.is_empty() {
            return Err(Error::MissingPayload);
        }
        if payload.contains('\n') {
            return Err(Error::UnexpectedNewline);
        } else {
            return Ok(Command::Publish(payload.trim().to_string()));
        }
    }

    if let Some(payload) = cmd.strip_prefix("RETRIEVE") {
        if !payload.is_empty() {
            return Err(Error::UnexpectedPayload);
        } else {
            return Ok(Command::Retrieve);
        }
    }

    Err(Error::UnknownCommand)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedNewline);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands

    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command

    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Retrieve);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command

    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::Publish("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_payload() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn dummy() {
        let line = " \n";
        assert_eq!(Some(" "), line.strip_suffix("\n"));
    }
}
