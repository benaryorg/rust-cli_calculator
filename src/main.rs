extern crate regex;

use regex::{Regex,Captures};

/// Calculates the result of the given calculation.
///
/// The calculation may only contain one type of arithmetic operations, either
/// `[+-]` or `[*/]` and may not contain braces.
/// The result will be calculated by replacing the first arithmetic operator and
/// the first two numbers with the result of this calculation.
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

/// Parses a calculation including the four basic arithmetic operations and
/// braces.
///
/// This function is responsible for dealing with the order of the operations.
/// It firstly recursively disassembles the passed string  based on the braces
/// contained.
/// These parts are being calculated and merged into the former string, which
/// then will not contain braces anymore.
/// Secondly it passes the multiplications and divisions to `calc` and lastly
/// resolves additions and subtractions using, again, `calc`.
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

/// Prepares the commandline arguments for usage with `parse` which then will be
/// called.
///
/// The format of `parse`s argument is the following:
///
/// - no spaces
fn main()
{
	let args:Vec<String>=std::env::args().skip(1).collect();
	let s=args.connect("").replace(" ","");
	println!("{}",parse(s));
}

