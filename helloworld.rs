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


fn main() {
    escopo();
    println!("Resultado da soma = {}", soma(21, -9))
}