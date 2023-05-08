use rust_stemmers::{Algorithm, Stemmer};
use unidecode::unidecode;

fn main() {
    // Exemplo de entrada de texto
    let text = "Olá! Este é um exemplo de texto para a apresentação de PCA. :)";

    // Lista de stopwords
    let stopwords = ["um", "de", "a"];

    // Pré-processa o texto
    let processed_text = preprocess_text(text, &stopwords);

    println!("{}", processed_text);
}

// Função para pré-processar o texto
fn preprocess_text(text: &str, stopwords: &[&str]) -> String {
    // Cria o stemmer para o português
    let stemmer = Stemmer::create(Algorithm::Portuguese);

    // Remover caracteres especiais e transformar em minúsculas
    let processed_text = text
        .to_lowercase() // transforma em minúsculas
        .split_whitespace() // divide em palavras
        .filter(|word| !stopwords.contains(&word)) // remove stopwords
        .map(|word| stemmer.stem(word).to_string()) // aplica o stemmer
        .map(|word| unidecode(word.as_str())) // Remover acentua;óes
        .collect::<Vec<String>>() // coleta em um vetor
        .join(" ") // junta novamente em uma String
        .replace(|c: char| !c.is_ascii_alphanumeric(), " "); // remove caracteres especiais

    processed_text
}
