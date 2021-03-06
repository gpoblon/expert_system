fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nb_args = args.len();
    let mut options = lib::options::Options::new();
    let mut launched = 0;

    let mut i = 1;
    while i < nb_args {
        let arg = &args[i];
        match arg.chars().next() {
            Some('-') => options.load(arg),
            _ => {
                lib::expert_system::run(arg, &options);
                launched += 1;
            }
        }
        i += 1;
    }
    if launched == 0 {
        println!("usage: ./expert_system [-giflc] [input_file ...]");
        println!("       -g : print ascii graph");
        println!("       -i : interactive fact validation");
        println!("       -f : print file");
        println!("       -l : print result into log file");
        println!("       -c : print only comments (only if -f inactive)\n");
        println!("       cargo run [-- -giflc] input_file ...");
        println!("       note: '--' allow cargo to load options");
    }
}
