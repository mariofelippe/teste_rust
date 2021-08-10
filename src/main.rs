use std::io;

fn main() {
    let nome: String;
    let idade: u8;
    let mut input = String::new(); 

    
    loop {
        println!("Infome seu nome:");
        match io::stdin().read_line(&mut input) {
            Ok(_x) => {
                if input.trim().len() > 0 {
                    nome = input.trim().to_string();
                    break;
                } else {
                    println!("Favor preencher a informação!");
                }
            }
            Err(e) => {
                println!("Erro ao pegar dados! {}", e);
            }
        }
    }

    loop {
        input = String::new();
        println!("Informe sua idade: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(x) = input.trim().parse::<u8>() {
                    if x > 0 {
                        idade = x;
                        break;
                    } else {
                        println!("A idade deve ser maior que zero!")
                    }
                } else {
                    println!("Idade inválida!")
                }
            }
            Err(e) => {
                println!("Erro ao pegar dados {}", e);
            }
        }
    }

    println!("Seu nome é {} e sua idade é {}. Bem vindo ao Rust!", nome, idade);
}
