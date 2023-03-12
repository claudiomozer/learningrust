const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 8;

fn soma(a:i32, b:i32) -> i32 {
    a + b
}

fn escopo() {
    println!("constante = {}", PI);
    println!("variavel global = {}, tamanho={}", VARIAVEL_GLOBAL, std::mem::size_of_val(&VARIAVEL_GLOBAL));

    let variavel:u8 = 128;
    println!("variavel = {}, tamanho={}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal antes do escopo = {}", decimal);

    {
        let decimal:f32 = 44.4;
        println!("decimal do escopo (shadowing) = {}", decimal);
    }

    println!("decimal depois do escopo = {}", decimal);

    let booleano = false;
    println!("Tamanho booleano = {}", std::mem::size_of_val(&booleano));

    let mut decimal_mutavel:f32 = 13.4;
    println!("Decimal antes da modificação {}. Tamanho {}", decimal_mutavel, std::mem::size_of_val(&decimal_mutavel));
    
    decimal_mutavel = 2.2;
    println!("Decimal após a modificação {}.", decimal_mutavel);

    let letra:char = 'C';
    println!("Char {}, Tamanho {}", letra, std::mem::size_of_val(&letra));
}

fn estruturas_condiciionais() {
    let idade: u8 = 15;
    let emancipado = true;

    if idade >= 18 {
        println!("Você possui {} anos, já é maior de idade", idade);
    } else if emancipado {
        println!("Você possui {} anos, é menor de idade, mas é emancipado", idade)
    } else {
        println!("Você possui {} anos, ainda é menor de idade", idade);
    }

    let maior_de_idade : &str = if idade >= 18 { "maior" } else { "menor" };

    println!("Você é {} de idade", maior_de_idade);

    let linguagem: &str = "Go";
    let proposito: &str = match linguagem {
        "PHP" => "Common webpages",
        "Go" => "Light and performatic microsservices",
        "Kotlin" | "Java" => "Android",
        "C" | "C++" => "Embeded systems",
        "Python" => "IA",
        "Rust" => "Performatic applications",
        _ => "Desconhecido"
    };

    println!("O propósito de {} é {}.", linguagem, proposito);
}

fn taboada() {
    let multiplicador:u8= 6;
    
    let mut count:u8 = 0;
    while count < 10 {
        count += 1;
        println!("{} x {} = {}", multiplicador, count, multiplicador*count);
    }

    count = 0;
    loop {
        count += 1;
        
        println!("{} x {} = {}", multiplicador, count, multiplicador*count);
        if count == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador*i);
    }
}

fn ownership() {
    let mut uma_string = String::from("Craudio");
    rouba(&mut uma_string);

    println!("{}", uma_string); 
}

fn rouba (string: &mut String){
    string.push_str(" Matador");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn main() {
    // tipos de dados e estruturas básicas
    escopo();
    // funções
    println!("Resultado da soma = {}", soma(21, -9));
    // if
    estruturas_condiciionais();

    // loops
    taboada();

    // ownership
    ownership();

    // pattern_matching
    pattern_matching();
}