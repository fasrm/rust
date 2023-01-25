use std::io;

fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    // declarando variáveis
    // são imutáveis
    let name= "Franciny";
    let age_example = 42;
    let mut sobrenome = "Salles"; // Aqui estou declarando que é mutável
    // age += 1; <- gera erro
    // name = "Andres"; <- GERARÁ UM ERRO

    // Tipos de Dados
    // Inteiros, Floats e Booleanos
    let x: u64 = 23;
    let f: f64 = 6.7; 
    let b: bool = true;

    sobrenome = "Rojas";

    let number1 = 5;
    let number2 = 5;

    if number1 > number2 {
        println!("{} > {}", number1, number2);
    } else {
        println!("{} < {}", number1, number2);
    }

    // Exemplo Input de Dados e Fluxo de Controle para Análise
    let mut numero1 = String::new();
    io::stdin().read_line(&mut numero1).expect("Erro ao ler numero1");

    let mut numero2 = String::new();
    io::stdin().read_line(&mut numero2).expect("Erro ao ler numero2");

    if convert_to_int(&numero1) > convert_to_int(&numero2) {
        println!("O numero {} eh maior que {}", numero1, numero2);
    } else {
        println!("O numero {} eh menor ou igual que {}", numero1, numero2);
    }
    
    println!("Hello, world");
    println!("Hello {}!", name);
    println!("Age {}", age_example);
    println!("Sobrenome: {}", sobrenome);
    println!("x: {}  f: {}  b: {}", x, f, b);
    
}

/*
1. Essa linha importa a biblioteca "io" do Rust, que fornece funções para entrada e saída de dados.

2. Essa linha define uma função chamada "convert_to_int" que recebe uma referência para uma string e retorna um valor inteiro (i32).

3. Essa linha cria uma variável local chamada "x" e atribui a ela o resultado da chamada de método "parse()" na string referenciada por "data_input". O método "parse()" tenta converter o conteúdo da string em um tipo especificado entre os sinais "::<>", nesse caso, um i32. O método "unwrap()" é chamado em seguida para obter o valor armazenado na variante "Ok" do tipo "Result" retornado pelo método "parse()". Se o método "parse()" retornar a variante "Err", o método "unwrap()" causará um panic.

4. Essa linha retorna o valor da variável "x".

5. Essa linha define a função principal do programa, que é executada quando o programa é iniciado.

6. Essa linha cria uma variável mutável chamada "number1" do tipo "String" e a inicializa com uma string vazia.

7. Essa linha chama o método "read_line()" da biblioteca "io", passando a referência para "number1" como argumento. O método "read_line()" lê uma linha de entrada do usuário até encontrar um caractere de nova linha. O método "expect()" é chamado em seguida para tratar qualquer erro que possa ocorrer durante a chamada ao método "read_line()". Se ocorrer um erro, o método "expect()" causará um panic com a mensagem de erro passada como argumento.

8. Essa linha é similar à linha 6, criando uma nova variável mutável chamada "number2".

9. Essa linha é similar à linha 7, lendo uma linha de entrada do usuário e armazenando-a em "number2".

10. Essa linha é um bloco de código que é executado se a condição for verdadeira. A condição é avaliada chamando a função "convert_to_int()" com as referências para "number1" e "number2" como argumentos e comparando se o valor retornado pela primeira chamada é maior que o valor retornado pela segunda. Se a condição for verdadeira, a mensagem "O número {} é maior que {}" será impressa. Caso contrário, a mensagem "O número {number1} é menor ou igual a {number2}" é impressa.


*/