use getter_derive::{length, AnswerFn, Getter};

#[length(10)]
#[derive(Getter, Default, AnswerFn)]
pub struct Point {
    x: u8,
    y: u8,
}
