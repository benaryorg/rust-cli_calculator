extern crate regex;

use regex::{Regex,Captures};

fn calc(equation:String)->String
{
	let mut equ=equation;
	let split=Regex::new(
		r"(?P<n1>(\d*\.)?\d+)(?P<char>[-+*/])(?P<n2>(\d*\.)?\d+)").unwrap();

	while split.is_match(equ.as_ref())
	{
		equ=split.replace(equ.as_ref(),|cap:&Captures|
		{
			let n1=cap.name("n1").unwrap().parse::<f64>().unwrap();
			let n2=cap.name("n2").unwrap().parse::<f64>().unwrap();
			//TODO: write back different for more accuracy
			format!("{}",match cap.name("char").unwrap()
			{
				"+" => n1+n2,
				"-" => n1-n2,
				"*" => n1*n2,
				"/" => n1/n2,
				_ => unreachable!(),
			})
		});
	}
	equ
}

fn parse(equation:String)->String
{
	let mut equ=equation;
	let braces=Regex::new(r"\((?P<res>([^()]*|(.*?\(.*?\))+.*?))\)").unwrap();
	let dot=Regex::new(r"((\d*\.)?\d+)([*/]((\d*\.)?\d+))+").unwrap();
	let line=Regex::new(r"((\d*\.)?\d+)([+-]((\d*\.)?\d+))+").unwrap();

	equ=braces.replace_all(equ.as_ref(),
		|cap:&Captures|parse(cap.name("res").unwrap().to_string()));
	equ=dot.replace_all(equ.as_ref(),
		|cap:&Captures|calc(cap.at(0).unwrap().to_string()));
	equ=line.replace_all(equ.as_ref(),
		|cap:&Captures|calc(cap.at(0).unwrap().to_string()));
	equ
}

fn main()
{
	let args:Vec<String>=std::env::args().skip(1).collect();
	let s=args.connect("").replace(" ","");
	//TODO: maybe call a function called `solve` and handle function calls like
	//		`ln`, `sin` and so on
	//		This could also read from a configuration file, adding more
	//		functions and user-defined functions
	println!("{}",parse(s));
}

