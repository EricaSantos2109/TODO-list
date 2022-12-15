fn main() {
    println!("Bem vindo ao TODO List");
    loop{
        if !create_todo(){
            break
        }   
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
 }

fn create_todo() -> bool{
    println!("Gostaria de criar um novo TODO? (s/n)");
    let anwser_todo = input();

    if anwser_todo == "s" || anwser_todo == "S"{
        println!("Qual TODO gostaria de criar?");
        let todo_user: String = input();
        println!("TODO: {}", todo_user);
        true
    } else if anwser_todo == "n" || anwser_todo == "N"{
        println!("OK!");
        false
    } else {
        println!("Entrada inválida! Tente novamente com a resposta s/n!");
        true
    }
}
