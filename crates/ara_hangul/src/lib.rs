#![allow(unused)]

pub struct Hangul(char);

impl From<Jamo> for Hangul {
	fn from(value: Jamo) -> Self { todo!() }
}

impl From<CompleteHangul> for Hangul {
	fn from(value: CompleteHangul) -> Self { todo!() }
}

pub struct Jamo(char);

impl From<Choseong> for Jamo {
	fn from(value: Choseong) -> Self { todo!() }
}

impl From<Jungseong> for Jamo {
	fn from(value: Jungseong) -> Self { todo!() }
}

impl From<Jongseong> for Jamo {
	fn from(value: Jongseong) -> Self { todo!() }
}

pub struct Choseong(char);

pub struct Jungseong(char);

pub struct Jongseong(char);

pub struct CompleteHangul(char);

pub struct DisassembledHangul {
	choseong: Choseong,
	junseong: Jungseong,
	jongseong: Option<Jongseong>,
}
