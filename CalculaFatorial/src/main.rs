use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada fatorial");
    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial);

    while entrada_int > 1 {
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int - 1;
    }
    println!("O fatorial eh {}", fatorial);
}

/*
use std::io: importa a biblioteca io para acessar funções de leitura e escrita de dados.

fn convert_to_int(data_input: & String) -> i32: declara a função convert_to_int que recebe uma string e retorna um inteiro. A string é passada como referência (&) para a função para evitar uma cópia desnecessária dos dados.

let x = data_input.trim().parse::<i32>().unwrap(): essa linha converte a string em um inteiro. O método trim remove os espaços em branco no início e no final da string. Em seguida, o método parse tenta realizar a conversão para o tipo especificado entre os sinais de maior e menor <>, no caso i32 (um inteiro de 32 bits). O método unwrap retorna o valor da operação caso ela tenha sido bem sucedida, ou lança um erro caso contrário.

let mut entrada_fatorial = String::new(): cria uma variável mutável (mut) chamada entrada_fatorial com valor inicial vazio (String::new()).

io::stdin().read_line(&mut entrada_fatorial).expect("Erro ao ler entrada_fatorial"): lê uma linha de entrada do usuário e armazena em entrada_fatorial. O método expect é usado para tratar possíveis erros na leitura dos dados.

let mut fatorial = 1: cria uma variável mutável fatorial com valor inicial 1.

let mut entrada_int = convert_to_int(&entrada_fatorial): cria uma variável mutável entrada_int e chama a função convert_to_int passando a string entrada_fatorial como argumento e armazena o resultado na variável mutável entrada_int.

while entrada_int > 1 Inicia um loop que continua enquanto entrada_int for maior que 1.

fatorial = fatorial * entrada_int; Atualiza o valor de fatorial para o valor atual de fatorial multiplicado por entrada_int.

entrada_int = entrada_int - 1; Diminui o valor de entrada_int em 1.

println!("O fatorial eh {}", fatorial); Imprime a mensagem "O fatorial é" seguida pelo valor atual de fatorial.
*/