mod tokenizer;
mod evaluator;
mod parser;

// expose evaluator::evaluate
pub use evaluator::evaluate;
pub use tokenizer::Token;