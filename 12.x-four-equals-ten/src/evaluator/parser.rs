use crate::util::find_token_from_position;
use super::tokenizer::Token;



pub fn parse(mut tokens: Vec<Token>) -> f32 {
	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now


	// search for parentheses
	let lparen_pos = find_token_from_position(&tokens, Token::LParen, 0);

	// if there is a set of parentheses
	if lparen_pos != usize::MAX {
		let rparen_pos = find_token_from_position(&tokens, Token::RParen, 0);


		// calculate number of expressions to evaluate inside the parentheses
		// position of tokens in parentheses = (lparen + 1, rparen - 1)
		// divide by 2 because yes idk how to articulate it it works
		let mut num_expressions = (rparen_pos - lparen_pos - 2) / 2;

		dbg!(&tokens);

		// loop over every expression
		while num_expressions > 0 {
			// check for multiplication/division
			let multiply_pos = find_token_from_position(&tokens, Token::Multiply, lparen_pos);
			let divide_pos = find_token_from_position(&tokens, Token::Divide, lparen_pos);

			// find which one comes first
			let mut operator_pos = std::cmp::min(multiply_pos, divide_pos);

			// if there's not multiplication/division, find whichever addition/subtraction comes first
			if operator_pos == usize::MAX {
				let add_pos = find_token_from_position(&tokens, Token::Add, lparen_pos);
				let subtract_pos = find_token_from_position(&tokens, Token::Subtract, lparen_pos);

				operator_pos = std::cmp::min(add_pos, subtract_pos);
			}

			// compute the expression
			let operation_value = evaluate_expression(&tokens[operator_pos], &tokens[operator_pos - 1], &tokens[operator_pos + 1]);

			// replace [..., operand_one, operation, operand_two, ...] with [..., result, ...]
			substitute_expression(&mut tokens, operator_pos, operation_value);

			num_expressions -= 1;
		}

		// remove unneeded parentheses
		remove_parentheses(&mut tokens);
	}



	// let input_len = tokens.len();

	// let num_expressions = (input_len - 1) / 2;

	// while num_expressions > 0 {


	// 	num_expressions;
	// }




	dbg!(&tokens);


	return 3f32;
}



fn substitute_expression(input: &mut Vec<Token>, operator_position: usize, value: f32) {
	let mut input_new: Vec<Token> = vec![];
	let input_len = input.len();

	// e.g. 1*(2/3)+4
	// operator_position = 5
	// before_expression = 1*(
	// after_expression = )+4
	let before_expression = &input[0..=(operator_position - 2)];
	let after_expression = &input[(operator_position + 2)..input_len];

	input_new.extend_from_slice(before_expression);
	input_new.push(Token::Number(value));
	input_new.extend_from_slice(after_expression);

	*input = input_new;
}



fn remove_parentheses(input: &mut Vec<Token>) {
	let mut input_new: Vec<Token> = vec![];
	let input_len = input.len();

	let lparen_pos = find_token_from_position(input, Token::LParen, 0);
	let rparen_pos = find_token_from_position(input, Token::RParen, 0);

	let before_paren = &input[0..=(lparen_pos - 1)];
	let paren_contents = &input[(lparen_pos + 1)..=(rparen_pos - 1)];
	let after_paren = &input[(rparen_pos + 1)..input_len];


	input_new.extend_from_slice(before_paren);
	input_new.extend_from_slice(paren_contents);
	input_new.extend_from_slice(after_paren);

	*input = input_new;
}



fn evaluate_expression(operation: &Token, operand_one: &Token, operand_two: &Token) -> f32 {
	let operand_one = value_from_token(operand_one);
	let operand_two = value_from_token(operand_two);

	return match operation {
		Token::Add => operand_one + operand_two,
		Token::Subtract => operand_one - operand_two,
		Token::Multiply => operand_one * operand_two,
		Token::Divide => operand_one / operand_two,

		_ => {
			panic!("invalid operation supplied to evaluate_expression!");
		}
	};
}



fn value_from_token(number: &Token) -> f32 {
	return match number {
		Token::Number(value) => *value,
		_ => panic!("value_from_token called with non-number!")
	};
}
