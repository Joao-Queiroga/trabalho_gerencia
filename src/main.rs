use std::fs::File;
use std::io::Write;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rand_num: u8 = rng.gen_range(0..=255);
    let mut file = File::create("random-number.txt").expect("Não conseguiu criar o arquivo");
    write!(file, "Número aleatório = {rand_num}").expect("Não conseguiu escrever no arquivo");
    println!("Número aleatório gerado e salvo em 'random_number.txt': {rand_num}",);
}

fn _soma(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::_soma;

    #[test]
    fn test_soma() {
        let a = 8;
        let b = 16;
        assert_eq!(_soma(a, b), 24);
    }
}
