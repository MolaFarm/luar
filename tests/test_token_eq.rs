use luar::lex::Token;


#[test]
fn test_eq_between_tokens (){
    assert_eq!(Token::Add,Token::Add);
    assert_eq!(Token::Integer(1),Token::Integer(1));

    assert_ne!(Token::Integer(1),Token::Integer(2));
    assert_ne!(Token::Integer(1),Token::String(String::from("value")));
    assert_ne!(Token::String(String::from("value")),Token::String(String::from("value2")))
}