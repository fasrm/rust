fn dobro(num: i32) -> i32 {
    return 2*num;
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b { 
        return a;
    }else{
        return b;
    }
}

fn alguma_fn(par_a: f32, parb_b: i128) -> f32 {
    println!("Essa funcao devolve um valor flutuante");
    //10 as f32
    let x = 10.1 as f32 * par_a + par_b as f32;
    x
}

fn main() {
    println!("O dobro de 5 eh: {}", dobro(5));
    println!("O maior numero entre 5 e 4 eh {}", maior(5,4));
}