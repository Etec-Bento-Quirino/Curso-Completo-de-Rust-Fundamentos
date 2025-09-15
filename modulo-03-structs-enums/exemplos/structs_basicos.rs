#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);

// Unit struct
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    println!("=== Exemplo: Structs Básicos ===");
    
    // Criando uma instância de User
    let mut user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("Usuário: {:?}", user1);
    
    // Modificando campos
    user1.email = String::from("alice.new@example.com");
    user1.sign_in_count += 1;
    
    println!("Usuário atualizado: {:?}", user1);
    
    // Usando função construtora
    let user2 = build_user(
        String::from("bob@example.com"),
        String::from("bob"),
    );
    println!("Novo usuário: {:?}", user2);
    
    // Struct update syntax
    let user3 = User {
        email: String::from("charlie@example.com"),
        username: String::from("charlie"),
        ..user1 // Copia os outros campos de user1
    };
    println!("Usuário com update syntax: {:?}", user3);
    
    // Trabalhando com Rectangle
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    
    let rect2 = Rectangle {
        width: 10.0,
        height: 40.0,
    };
    
    println!("Área do retângulo 1: {}", rect1.area());
    println!("Área do retângulo 2: {}", rect2.area());
    println!("Perímetro do retângulo 1: {}", rect1.perimeter());
    println!("Rect1 pode conter rect2? {}", rect1.can_hold(&rect2));
    println!("Rect1 é quadrado? {}", rect1.is_square());
    
    // Associated function
    let square = Rectangle::square(10.0);
    println!("Quadrado: {:?}, Área: {}", square, square.area());
    println!("É quadrado? {}", square.is_square());
    
    // Tuple structs
    let point = Point { x: 0.0, y: 0.0 };
    println!("Ponto: {:?}", point);
    
    let color = Color(255, 0, 0);
    println!("Cor RGB: {:?}", color);
    
    // Unit struct
    let always_equal = AlwaysEqual;
    println!("Unit struct: {:?}", always_equal);
    
    // Demonstração de métodos com parâmetros
    let rect3 = Rectangle {
        width: 20.0,
        height: 20.0,
    };
    
    println!("Comparação de retângulos:");
    println!("Rect1 pode conter rect3? {}", rect1.can_hold(&rect3));
    println!("Rect3 pode conter rect1? {}", rect3.can_hold(&rect1));
    
    // Demonstração de métodos que consomem self
    let rect4 = Rectangle {
        width: 5.0,
        height: 5.0,
    };
    
    println!("Área calculada: {}", rect4.area());
    // rect4 ainda pode ser usado porque area() não consome self
    
    // Demonstração de métodos que modificam self
    let mut rect5 = Rectangle {
        width: 10.0,
        height: 10.0,
    };
    
    println!("Antes: {:?}", rect5);
    // Aqui poderíamos ter um método que modifica o retângulo
    println!("Depois: {:?}", rect5);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Função que trabalha com structs
fn calcular_distancia(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

// Função que retorna struct
fn criar_ponto_origem() -> Point {
    Point { x: 0.0, y: 0.0 }
}

// Função que modifica struct
fn mover_ponto(p: &mut Point, dx: f64, dy: f64) {
    p.x += dx;
    p.y += dy;
}
