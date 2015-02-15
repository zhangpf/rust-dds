fn print_usage(name: &str) {
	println!("Usage: {} [-c preprocessor-path]\n\
			         [-n <include-suffix>] [-I path] [-D macro[=definition]] [-S | -C] \n\
			         [-l (c | c++ | cpp | isocpp | isoc++ | cs | java)] [-j [old]:<new>] [-d directory] [-i] \n\
			         [-P dll_macro_name[,<h-file>]] [-o (dds-types | custom-psm | no-equality)] <filename>\n", name);
}

fn main() {
	let ret: int = 
	print_usage("");
}