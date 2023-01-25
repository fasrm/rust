use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {

    let mut medias = String::new();
    io::stdin().read_line(&mut medias).expect("Erro ao ler medias");
    let mut soma_rec = 0;
    let mut i = 0;

    while convert_to_int(&medias) > i{
        let mut media_aluno = String::new();
        io::stdin().read_line(&mut media_aluno).expect("Erro ao ler media_aluno");
        i += 1;
        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
            soma_rec += 1;
        }
    }

    println!("O numero de alunos em recuperacao eh {}", soma_rec);
    
}

/*
Esse código lê várias entradas de notas de alunos e conta quantos deles têm uma nota entre 3 e 6 (inclusive). A função convert_to_int é usada para converter os dados de entrada, que são strings, em inteiros. O loop while lê as notas de cada aluno e adiciona 1 à variável soma_rec se a nota do aluno for maior ou igual a 3 e menor que 6. Por fim, o programa imprime o número de alunos com notas entre 3 e 6.



use std::io;
Essa linha importa o módulo io da biblioteca padrão do Rust, que fornece funções para entrada e saída de dados.



fn convert_to_int(data_input: &String) -> i32


Essa linha define uma função chamada convert_to_int que recebe uma referência para uma String (data_input) e retorna um i32.



let x = data_input.trim().parse::<i32>().unwrap();


Essa linha cria uma variável x e atribui a ela o resultado da conversão da string data_input para um inteiro. O método trim é usado para remover os espaços em branco no início e no final da string. O método parse é usado para tentar converter a string para um inteiro. O sufixo ::<i32> é usado para especificar que o inteiro deve ser do tipo i32. O método unwrap é usado para obter o valor dentro do Result retornado pelo método parse. Se a conversão falhar, o programa será encerrado com uma mensagem de erro.



 x
Essa linha retorna o valor da variável x, que é a string convertida para um inteiro.

fn main() {

Essa linha define a função principal do programa.



let mut medias = String::new();
io::stdin().read_line(&mut medias).expect("Erro ao ler media_aluno");


Essa linha cria uma variável mutável chamada medias e atribui a ela uma nova string vazia. Em seguida, usa a função read_line do módulo io para ler uma linha de entrada do usuário e armazená-la na variável medias. O método expect é usado para lidar com possíveis erros durante a leitura da entrada. Se um erro ocorrer, a mensagem de erro "Erro ao ler media_aluno" será exibida e o programa será encerrado.



let mut soma_rec = 0;
let mut i = 0;


Essas linhas criam duas variáveis mutáveis: soma_rec e i. A variável soma_rec é inicializada com 0 e i é inicializada com 0.



while convert_to_int(&medias) > i {


Essa linha inicia um loop while que continuará enquanto o resultado da função convert_to_int aplicada à variável medias for maior que i.



let mut media_aluno = String::new();
io::stdin().read_line(&mut media_aluno).expect("Erro ao ler media_aluno");
i += 1;


Essas linhas criam uma variável mutável chamada media_aluno e atribui a ela uma nova string vazia.



if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6{
            soma_rec += 1;
        }


Essa linha verifica se o resultado da função convert_to_int aplicada à variável media_aluno é maior ou igual a 3 e menor que 6. Se essa condição for verdadeira, a variável soma_rec é incrementada em 1.



println!("O numero de alunos em rec eh {}", soma_rec);


Essa linha usa a macro println! para imprimir a mensagem "O número de alunos em rec é" seguida pelo valor da variável soma_rec.
*/