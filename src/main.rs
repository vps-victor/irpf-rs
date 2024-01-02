use std::io;

fn main() {
    // APRESENTAÇÃO DO PROGRAMA
    println!("CALCULADORA DE IRPF");
    println!("Digite o quanto ganha (decimais com ponto):");

    // Entre os dados do usuário
    let mut valor_input = String::new();
    io::stdin().read_line(&mut valor_input).expect("Falha ao ler a linha");

    // Trata espaço e entrada do usuário
    match valor_input.trim().parse::<f64>() {
        Ok(num) if num > 0.0 => {
            // Chama a função calcular_aliquota com o número inserido
            calcular_aliquota(num);
        }
        Ok(_) => println!("Por favor, insira um número positivo"),
        Err(_) => println!("Por favor, insira um número válido"),
    }
}

// Calcula a aliquota do valor
fn calcular_aliquota(valor_input: f64) {
    if valor_input <= 24511.92 {
        resultado_aliquota(valor_input, 0.0);
    } else if valor_input <= 33919.80 {
        resultado_aliquota(valor_input, 0.075);
    } else if valor_input <= 45012.60 {
        resultado_aliquota(valor_input, 0.15);
    } else if valor_input <= 55976.16 {
        resultado_aliquota(valor_input, 0.225);
    } else {
        resultado_aliquota(valor_input, 0.275);
    }
}

// Função para representar o cálculo da alíquota
fn resultado_aliquota(valor_input: f64, aliquota: f64) {
    let valor_a_pagar = valor_input * aliquota;
    let formatted_valor_a_pagar = format!("{:.2}", valor_a_pagar);
    let formatted_aliquota_percent = format!("{:.1}", aliquota * 100.0);

    println!(
        "Você pagará {} de imposto, pois o valor {} tem {}% de alíquota",
        formatted_valor_a_pagar, valor_input, formatted_aliquota_percent
    );
}

