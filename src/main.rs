// Define a trait for Homomorphisms
trait Homomorphism<T> {
    fn map(&self, input: T) -> T;
}

// Struct representing a homomorphism from Z -> Z_6
struct ModHomomorphism {
    modulus: i32,
}

impl Homomorphism<i32> for ModHomomorphism {
    fn map(&self, input: i32) -> i32 {
        input % self.modulus
    }
}

fn main() {
    let homomorphism = ModHomomorphism { modulus: 6 };
    let a = 14;
    let b = 5;
    
    // Test the homomorphism property: φ(a + b) = φ(a) + φ(b) mod 6
    let mapped_sum = homomorphism.map(a + b);  // φ(14 + 5)
    let sum_of_mapped = (homomorphism.map(a) + homomorphism.map(b)) % homomorphism.modulus; // (φ(14) + φ(5)) % 6

    println!(
        "φ({}) = {}, φ({}) = {}, φ({} + {}) = {}, (φ({}) + φ({})) % 6 = {}",
        a,
        homomorphism.map(a),
        b,
        homomorphism.map(b),
        a,
        b,
        mapped_sum,
        a,
        b,
        sum_of_mapped
    );

    // Check if the homomorphism property holds
    assert_eq!(mapped_sum, sum_of_mapped);
    println!("Homomorphism property verified!");
}
