pub fn validar_cpf(cpf: &str) -> bool {
        
    // Remove caracteres não numéricos (pontos e traço)
    let cpf: Vec<u32> = cpf
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    // CPF deve ter exatamente 11 dígitos
    if cpf.len() != 11 {
        return false;
    }

    // Rejeita sequências de dígitos repetidos (ex: 111.111.111-11)
    if cpf.iter().all(|&d| d == cpf[0]) {
        return false;
    }

    // Calcula o primeiro dígito verificador
    let calcular_digito = |cpf: &[u32], peso_inicial: u32| -> u32 {
        let soma: u32 = cpf
            .iter()
            .enumerate()
            .map(|(i, &d)| d * (peso_inicial - i as u32))
            .sum();

        let resto = soma % 11;
        if resto < 2 { 0 } else { 11 - resto }
    };

    let primeiro_digito = calcular_digito(&cpf[0..9], 10);
    if primeiro_digito != cpf[9] {
        return false;
    }

    let segundo_digito = calcular_digito(&cpf[0..10], 11);
    if segundo_digito != cpf[10] {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_cpf_valido() {
        assert!(validar_cpf("529.982.247-25"));
        assert!(validar_cpf("52998224725"));
    }

    #[test]
    fn testa_cpf_invalido() {
        assert!(!validar_cpf("111.111.111-11")); // repetido
        assert!(!validar_cpf("123.456.789-00")); // dígitos errados
        assert!(!validar_cpf("123"));             // tamanho errado
    }
}


