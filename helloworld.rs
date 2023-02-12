fn main() {  
    let variavel:u8 = 128;
    println!("variavel = {}, tamanho={}", variavel, std::mem::size_of_val(&variavel));
}