use blockchain_rust::{blockchain::Blockchain, cli::Cli};

fn main() {
    let bc = Blockchain::new();
    let cli = Cli { bc: &bc };
    cli.run();
}
