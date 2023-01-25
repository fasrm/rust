fn main() {
    for i in 1..10 {
        println!("{}", i); // contando de 1 a 9
    }

    let faixa = 20..29;
    for i in faixa {
        println!("{}", i)
    }

    // Definindo um vetor
    let animais = vec!["Coelho", "Gato", "Macaco"];

    for a in animais {
        println!("{}", a);
    }
}
