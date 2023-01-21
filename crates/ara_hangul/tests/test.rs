use ara_hangul::*;

mod is_hangul {
	use super::*;

	#[test]
	fn must_ignore_ascii_characters() {
		assert!(!Hangul::is_hangul(
			char::from_u32(0).unwrap()
		));
		assert!(!Hangul::is_hangul(
			char::from_u32(1).unwrap()
		));
		assert!(!Hangul::is_hangul('A'));
	}

	fn must_include_complete_hangul() {
		assert!(Hangul::is_hangul('가'));
		assert!(Hangul::is_hangul('각'));
		assert!(Hangul::is_hangul('각'));
		assert!(Hangul::is_hangul('힣'));
	}

	fn must_include_partial_characters() {
		assert!(Hangul::is_hangul('ㄱ'));
		assert!(Hangul::is_hangul('ㅏ'));
		assert!(Hangul::is_hangul('ㄿ'));
	}

	fn must_ignore_old_hangul_characters() {
		assert!(!Hangul::is_hangul('ㅹ'));
		assert!(!Hangul::is_hangul('ᄭ'));
	}

	fn must_ignore_hcf() { assert!(!Hangul::is_hangul('\u{115F}')) }
}
