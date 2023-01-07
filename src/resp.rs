use crate::resp::RESPType;

#[derive(Debug)]
pub enum CommandType {
    Ping,
    Echo,
    None,
}

impl CommandType {
    pub fn get_type(command_text: String) -> CommandType {
        match command_text.to_lowercase().as_str() {
            "ping" => CommandType::Ping,
            "echo" => CommandType::Echo,
            _ => CommandType::None,
        }
    }

    pub fn handle_command(&self, args: Vec<&RESPType>) -> Vec<u8> {
        let hard_coded = "+PONG\r\n";

        match self {
            Self::Ping | Self::Echo => {
                println!("here");
                if args.len() == 0 {
                    return hard_coded.as_bytes().to_vec();
                }
                let args = args.get(0).unwrap();
                return args.pack();
            }
            _ => {
                return hard_coded.as_bytes().to_vec();
            }
        }
    }
}