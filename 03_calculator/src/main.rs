use std::io;

fn main() {
   
    println!("Digite o primeiro valor: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("erro ao ler o valor");
    let x: f64 = input1.trim().parse().expect("insira um número válido");

    println!("Digite o segundo valor: ");
    let mut input2 = String::new();
    io::stdin().
            read_line(&mut input2).
                expect("erro ao ler o valor");
    let y: f64 = input2.
        trim().
            parse().
                expect("insira um número válido");

    let difference = x - y;
    let sum = x + y;
    let product = x * y;
    let division = x / y;

    println!( "Subtração: {:.2}\nSoma: {:.2}\nProduto: {:.2}\nDivisão: {:.2}", difference, sum, product, division );
}
