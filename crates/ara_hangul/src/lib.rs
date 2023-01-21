#![allow(unused)]
#![allow(clippy::missing_safety_doc)]

pub trait FromCharUnchecked: TryFrom<char> {
	unsafe fn from_char_unchecked(value: char) -> Self;
}

pub trait ExtractJamo {
	fn extract_choseong(&self) -> Option<Choseong>;
	fn extract_jungseong(&self) -> Option<Jungseong>;
	fn extract_jongseong(&self) -> Option<Jongseong>;
}

pub trait Disassemble {
	type Output;

	fn disassemble(&self) -> Self::Output;
}

pub trait Assemble {
	type Output;

	fn assemble(&self) -> Self::Output;
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

impl TryFrom<char> for Hangul {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for Hangul {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
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

impl TryFrom<char> for Jamo {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl TryFrom<Hangul> for Jamo {
	type Error = ();

	fn try_from(value: Hangul) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for Jamo {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
}

pub struct Choseong(char);

impl Choseong {
	fn is_choseong(value: char) -> bool { todo!() }
}

impl TryFrom<char> for Choseong {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl TryFrom<Jamo> for Choseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for Choseong {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
}

pub struct Jungseong(char);

impl Jungseong {
	fn is_jungseong(value: char) -> bool { todo!() }
}

impl TryFrom<char> for Jungseong {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl TryFrom<Jamo> for Jungseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for Jungseong {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
}

pub struct Jongseong(char);

impl Jongseong {
	fn is_jongseong(value: char) -> bool { todo!() }
}

impl TryFrom<char> for Jongseong {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl TryFrom<Jamo> for Jongseong {
	type Error = ();

	fn try_from(value: Jamo) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for Jongseong {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
}

pub struct CompleteHangul(char);

impl CompleteHangul {
	fn is_complete_hangul(value: char) -> bool { todo!() }

	fn disassemble(&self) -> DisassembledHangul { todo!() }
}

impl TryFrom<char> for CompleteHangul {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> { todo!() }
}

impl TryFrom<Hangul> for CompleteHangul {
	type Error = ();

	fn try_from(value: Hangul) -> Result<Self, Self::Error> { todo!() }
}

impl FromCharUnchecked for CompleteHangul {
	unsafe fn from_char_unchecked(value: char) -> Self { todo!() }
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

impl DisassembledHangul {
	fn assemble(&self) -> CompleteHangul { todo!() }
}

impl From<(Choseong, Jungseong, Jongseong)> for DisassembledHangul {
	fn from(value: (Choseong, Jungseong, Jongseong)) -> Self { todo!() }
}

impl From<(Choseong, Jungseong, Option<Jongseong>)> for DisassembledHangul {
	fn from(value: (Choseong, Jungseong, Option<Jongseong>)) -> Self { todo!() }
}

impl TryFrom<[Jamo; 3]> for DisassembledHangul {
	type Error = ();

	fn try_from(value: [Jamo; 3]) -> Result<Self, Self::Error> { todo!() }
}

impl From<DisassembledHangul> for (Choseong, Jungseong, Jongseong) {
	fn from(value: DisassembledHangul) -> Self { todo!() }
}

impl From<DisassembledHangul> for [Jamo; 3] {
	fn from(value: DisassembledHangul) -> Self { todo!() }
}

impl Disassemble for String {
	type Output = Vec<Jamo>;

	fn disassemble(&self) -> Self::Output { todo!() }
}

impl Disassemble for Vec<char> {
	type Output = Vec<Jamo>;

	fn disassemble(&self) -> Self::Output { todo!() }
}

impl Assemble for Vec<char> {
	type Output = Vec<char>;

	fn assemble(&self) -> Self::Output { todo!() }
}

impl Disassemble for Vec<Hangul> {
	type Output = Vec<Jamo>;

	fn disassemble(&self) -> Self::Output { todo!() }
}

impl Assemble for Vec<Hangul> {
	type Output = Vec<Hangul>;

	fn assemble(&self) -> Self::Output { todo!() }
}

impl Disassemble for Vec<CompleteHangul> {
	type Output = Vec<Jamo>;

	fn disassemble(&self) -> Self::Output { todo!() }
}

impl Assemble for Vec<Jamo> {
	type Output = Vec<Hangul>;

	fn assemble(&self) -> Self::Output { todo!() }
}
pub struct JamoSequence;

impl Iterator for JamoSequence {
	type Item = Jamo;

	fn next(&mut self) -> Option<Self::Item> { todo!() }
}
