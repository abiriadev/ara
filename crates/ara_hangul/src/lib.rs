#![allow(unused)]

pub struct Hangul(char);

pub struct Jamo(char);

pub struct Choseong(char);

pub struct Jungseong(char);

pub struct Jongseong(char);

pub struct CompleteHangul(char);

pub struct DisassembledHangul {
	choseong: Choseong,
	junseong: Jungseong,
	jongseong: Option<Jongseong>,
}
