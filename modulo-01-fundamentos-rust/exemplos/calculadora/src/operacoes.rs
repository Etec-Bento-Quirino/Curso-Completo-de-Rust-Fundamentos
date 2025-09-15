pub fn somar(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtrair(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

pub fn dividir(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Erro: Divisão por zero!");
        0.0
    }
}

pub fn potencia(base: f64, expoente: f64) -> f64 {
    base.powf(expoente)
}

pub fn raiz_quadrada(numero: f64) -> f64 {
    if numero >= 0.0 {
        numero.sqrt()
    } else {
        println!("Erro: Raiz quadrada de número negativo!");
        0.0
    }
}

pub fn modulo(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a % b
    } else {
        println!("Erro: Módulo por zero!");
        0.0
    }
}

pub fn logaritmo(numero: f64) -> f64 {
    if numero > 0.0 {
        numero.ln()
    } else {
        println!("Erro: Logaritmo de número não positivo!");
        0.0
    }
}

pub fn seno(angulo_graus: f64) -> f64 {
    let angulo_radianos = angulo_graus.to_radians();
    angulo_radianos.sin()
}

pub fn cosseno(angulo_graus: f64) -> f64 {
    let angulo_radianos = angulo_graus.to_radians();
    angulo_radianos.cos()
}

pub fn tangente(angulo_graus: f64) -> f64 {
    let angulo_radianos = angulo_graus.to_radians();
    angulo_radianos.tan()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somar() {
        assert_eq!(somar(2.0, 3.0), 5.0);
        assert_eq!(somar(-1.0, 1.0), 0.0);
        assert_eq!(somar(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_subtrair() {
        assert_eq!(subtrair(5.0, 3.0), 2.0);
        assert_eq!(subtrair(1.0, 1.0), 0.0);
        assert_eq!(subtrair(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_multiplicar() {
        assert_eq!(multiplicar(4.0, 5.0), 20.0);
        assert_eq!(multiplicar(-2.0, 3.0), -6.0);
        assert_eq!(multiplicar(0.0, 100.0), 0.0);
    }

    #[test]
    fn test_dividir() {
        assert_eq!(dividir(10.0, 2.0), 5.0);
        assert_eq!(dividir(15.0, 3.0), 5.0);
        assert_eq!(dividir(7.0, 0.0), 0.0); // Divisão por zero retorna 0
    }

    #[test]
    fn test_potencia() {
        assert_eq!(potencia(2.0, 3.0), 8.0);
        assert_eq!(potencia(5.0, 2.0), 25.0);
        assert_eq!(potencia(10.0, 0.0), 1.0);
    }

    #[test]
    fn test_raiz_quadrada() {
        assert_eq!(raiz_quadrada(9.0), 3.0);
        assert_eq!(raiz_quadrada(16.0), 4.0);
        assert_eq!(raiz_quadrada(0.0), 0.0);
        assert_eq!(raiz_quadrada(-1.0), 0.0); // Número negativo retorna 0
    }

    #[test]
    fn test_seno() {
        assert_eq!(seno(0.0), 0.0);
        assert_eq!(seno(90.0), 1.0);
        assert_eq!(seno(180.0), 0.0);
    }

    #[test]
    fn test_cosseno() {
        assert_eq!(cosseno(0.0), 1.0);
        assert_eq!(cosseno(90.0), 0.0);
        assert_eq!(cosseno(180.0), -1.0);
    }
}
