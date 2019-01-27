fn main(){
	let mut code ="fn main(){\n\tlet mut code =\"\".to_string();\n\tlet s={\n\t\tlet mut temp=code.clone();\n\t\tfor (b,a) in vec![(\"\\\\\",\"\\\\\\\\\"),(\"\\\"\",\"\\\\\\\"\"),(\"\\n\",\"\\\\n\"),(\"\\t\",\"\\\\t\")]{\n\t\t\ttemp = temp.replace(b,a);\n\t\t}\n\t\ttemp\n\t};\n\tcode.insert_str(27,s.as_str());\n\tprintln!(\"{}\",code);\n}".to_string();
	let s={
		let mut temp=code.clone();
		for (b,a) in vec![("\\","\\\\"),("\"","\\\""),("\n","\\n"),("\t","\\t")]{
			temp = temp.replace(b,a);
		}
		temp
	};
	code.insert_str(27,s.as_str());
	println!("{}",code);
}
