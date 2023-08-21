#[derive(Debug)]
pub struct SyntaxError {
    code: i32,
    message: String,
    file_name: String,
}
