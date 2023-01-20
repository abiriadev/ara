#![allow(unused)]

pub trait ExtractJamo {
	fn extract_choseong(&self) -> Option<Choseong>;
	fn extract_jungseong(&self) -> Option<Jungseong>;
	fn extract_jongseong(&self) -> Option<Jongseong>;
}

pub struct Hangul(char);

impl Hangul {
	fn is_hangul(value: char) -> bool { todo!() }
}

impl From<Jamo> for Hangul {
	fn from(value: Jamo) -> Self { todo!() }
}

impl From<CompleteHangul> for Hangul {
	fn from(value: CompleteHangul) -> Self { todo!() }
}

impl ExtractJamo for Hangul {
	fn extract_choseong(&self) -> Option<Choseong> { todo!() }

	fn extract_jungseong(&self) -> Option<Jungseong> { todo!() }

	fn extract_jongseong(&self) -> Option<Jongseong> { todo!() }
}

pub struct Jamo(char);

impl Jamo {
	fn is_jamo(value: char) -> bool { todo!() }
}

impl From<Choseong> for Jamo {
	fn from(value: Choseong) -> Self { todo!() }
}

impl From<Jungseong> for Jamo {
	fn from(value: Jungseong) -> Self { todo!() }
}

impl From<Jongseong> for Jamo {
	fn from(value: Jongseong) -> Self { todo!() }
}

impl TryFrom<Hangul> for Jamo {
	type Error = ();

	fn try_from(value: Hangul) -> Result<Self, Self::Error> { todo!() }
}

pub struct Choseong(char);

impl Choseong {
	fn is_choseong(value: char) -> bool { todo!() }
}

impl TryFrom<Jamo> for Choseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

pub struct Jungseong(char);

impl Jungseong {
	fn is_jungseong(value: char) -> bool { todo!() }
}

impl TryFrom<Jamo> for Jungseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

pub struct Jongseong(char);

impl Jongseong {
	fn is_jongseong(value: char) -> bool { todo!() }
}

impl TryFrom<Jamo> for Jongseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

pub struct CompleteHangul(char);

impl CompleteHangul {
	fn is_complete_hangul(value: char) -> bool { todo!() }
}

impl TryFrom<Hangul> for CompleteHangul {
	type Error = ();

	fn try_from(value: Hangul) -> Result<Self, Self::Error> { todo!() }
}

impl ExtractJamo for CompleteHangul {
	fn extract_choseong(&self) -> Option<Choseong> { todo!() }

	fn extract_jungseong(&self) -> Option<Jungseong> { todo!() }

	fn extract_jongseong(&self) -> Option<Jongseong> { todo!() }
}

pub struct DisassembledHangul {
	choseong: Choseong,
	junseong: Jungseong,
	jongseong: Option<Jongseong>,
}
