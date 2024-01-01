# (EM ANDAMENTO) Calculadora de Imposto de Renda de Pessoa Física (IRPF)   
[![Rust Community](https://img.shields.io/badge/Rust_Community%20-Join_us-brightgreen?style=plastic&logo=rust)](https://www.rust-lang.org/community)

Calculadora simples para auxiliar no processo de Declaração (Anual) de Imposto de Renda Pessoa Física (DIRPF).

IRPF segue o formato **simplificado**, empregando dedução padrão de sobre o total dos rendimentos tributáveis auferidos durante o ano de 2023.

# Como é feito o cálculo do programa
_As duas tabelas abaixo foram retiradas da fonte: https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas/2023_

A partir de maio de 2023
| Base de cálculo                | Alíquota | Dedução   |
|--------------------------------|----------|-----------|
| Até R$ 2.112,00                | -        | -         |
| De R$ 2.112,01 até R$ 2.826,65 | 7,5%     | R$ 158,40 |
| De R$ 2.826,66 até R$ 3.751,05 | 15,0%    | R$ 370,40 |
| De R$ 3.751,06 até R$ 4.664,68 | 22,5%    | R$ 651,73 |
| Acima de R$ 4.664,68           | 27,5%    | R$ 884,96 |


_Rendimentos previdenciários isentos para maiores de 65 anos: R$ 1.903,98
Dedução mensal por dependente: R$ 189,59
Limite mensal de desconto simplificado: R$ 528,00_

De janeiro a abril de 2023.
| Base de cálculo                | Alíquota | Dedução   |
|--------------------------------|----------|-----------|
| Até R$ 1.903,98                | -        | -         |
| De R$ 1.903,99 até R$ 2.826,65 | 7,5%     | R$ 142,80 |
| De R$ 2.826,66 até R$ 3.751,05 | 15,0%    | R$ 354,80 |
| De R$ 3.751,06 até R$ 4.664,68 | 22,5%    | R$ 636,13 |
| Acima de R$ 4.664,68           | 27,5%    | R$ 869,36 |

_Rendimentos previdenciários isentos para maiores de 65 anos: R$ 1.903,98
Dedução mensal por dependente: R$ 189,59_

Incidência anual: a partir do exercício 2024 (ano-calendário 2023).
| Base de cálculo                  | Alíquota |    Dedução   |
|----------------------------------|:--------:|:------------:|
| Até R$ 24.511,92                 |     -    |       -      |
| De R$ 24.511,93 até R$ 33.919,80 |   7,5%   |  R$ 1.838,39 |
| De R$ 33.919,81 até R$ 45.012,60 |   15,0%  |  R$ 4.382,38 |
| De R$ 45.012,61 até R$ 55.976,16 |   22,5%  |  R$ 7.758,32 |
| Acima de R$ 55.976,16            |   27,5%  | R$ 10.557,13 |

_Dedução anual por dependente: R$ 2.275,08
Limite anual de despesa com instrução: R$ 3.561,50
Limite anual de desconto simplificado: R$ 16.754,34_

# Outros projetos
_Caso esteja procurando Imposto de Renda de Pessoa Jurídica (IRPJ): [Acesse (EM ANDAMENTO)]()_

