use std::{
    error::Error,
    io::{ErrorKind, Stdin, Stdout, Write},
};

fn main() {
    println!("Bem vindo ao TODO List");
    loop {
        println!("Gostaria de criar um novo TODO? (s/n)");
        let mut terminal = Terminal::new();

        match terminal.should_create_todo() {
            Ok(should_create) =>{
                if !should_create{
                    println!("OK Finalizando o programa!");
                    break;
                }
            }
            Err(error) => {
                println!("{}", show_error(error));
                continue;
            }
        }

        match terminal.ask_for_new_todo(){
            Ok(new_todo) => terminal.show_todo(&new_todo),
            Err(error) => {
                println!("{}", show_error(error));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    pub fn new(message: String) -> Todo {
        Todo { message }
    }
}

enum TerminalError {
    Stdin(std::io::Error),
    Stdout(std::io::Error),
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn should_create_todo(&mut self) -> Result<bool, TerminalError> {
        loop {
            let mut buf = String::new();

            match self.stdin.read_line(&mut buf) {
                Ok(_) => (),
                Err(error) => return Err(TerminalError::Stdin(error)),
            }

            let input = buf.trim().to_string();

            if input == "s" {
                return Ok(true);
            } else if input == "n" {
                return Ok(false);
            } else {
                return Err(TerminalError::Stdin(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "Entrada inválida, tente novamente com s/n!",
                )));
            }
        }
    }

    fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        let mut buf = String::new();
        writeln!(self.stdout, "Qual TODO gostaria de criar?").unwrap();
        
        match self.stdin.read_line(&mut buf) {
            Ok(_) => (),
            Err(error) => return Err(TerminalError::Stdin(error)),
        }

        let input = buf.trim().to_string();

        return Ok(Todo::new(input));
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "Sua mensagem: {}", todo.message).unwrap();
    }
}

fn show_error(error: TerminalError) -> String{
    
    match error {
        TerminalError::Stdin(error) => format!("Erro de entrada: {}", error),
        TerminalError::Stdout(error) => format!("Erro de saída: {}", error)
    }
}