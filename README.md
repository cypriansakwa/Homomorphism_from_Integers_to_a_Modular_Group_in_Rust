## Overview
This Rust program shows how to implement a homomorphism from the set of integers $\mathbb{Z}$ to the integers modulo $n$ $\mathbb{Z}_n$. It defines a trait for homomorphisms and provides a specific implementation that maps integers to their equivalence classes modulo $6$. The concept of a homomorphism is important in abstract algebra because it preserves the structure of two algebraic systems.
## How it works
- **Homomorphism Trait:** The program defines a generic Homomorphism<T>  trait with a single method map(&self, input: T) -> T. This method represents the homomorphism function that maps an element of type T to another element of the same type.
- **ModHomomorphism Struct:** A struct ModHomomorphism is defined to represent a homomorphism from $\mathbb{Z}$ to $\mathbb{Z}_6$. The struct contains a field modulus, which is used to perform the modulo operation.
- **Implementation Details:** The trait Homomorphism<i32> is implemented for ModHomomorphism, where the map function performs the modulo operation:
>```
>impl Homomorphism<i32> for ModHomomorphism {
>fn map(&self, input: i32) -> i32 {
> input % self.modulus
>}
>}
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.
## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $q,n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
## Example 
- The main function tests the homomorphism property: $\phi(a+b)=\phi(a)+\phi(b)\bmod 6$ where $\phi$ is the homomorphism function, $a$ and $b$ are integers, and $6$ is the modulus.
 >```
> fn main() {
>  let homomorphism = ModHomomorphism { modulus: 6 };
>  let a = 14;
>  let b = 5;
>  // Test the homomorphism property
> let mapped_sum = homomorphism.map(a + b);
> let sum_of_mapped = (homomorphism.map(a) + homomorphism.map(b)) % homomorphism.modulus;
>  println!( "\u{03C6}({}) = {}, \u{03C6}({}) = {}, \u{03C6}({} + {}) = {}, (\u{03C6}({}) + \u{03C6}({})) % 6 = {}",
>   a,
>   homomorphism.map(a),
>       b,
>       homomorphism.map(b),
>       a,
>       b,
>       mapped_sum,
>       a,
>        b,
>        sum_of_mapped
>        );
>       // Check if the homomorphism property holds
>     assert_eq!(mapped_sum, sum_of_mapped);
>     println!("Homomorphism property verified!");
>}
## Expected Output
- When running the program, you should see output similar to:
 >```
> φ(14) = 2, φ(5) = 5, φ(14 + 5) = 1, (φ(14) + φ(5)) % 6 = 1
> Homomorphism property verified!
- This confirms that the homomorphism property holds for the given integers and modulus.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Homomorphism_from_Integers_to_a_Modular_Group_in_Rust.git
   cd Homomorphism_from_Integers_to_a_Modular_Group_in_Rust
