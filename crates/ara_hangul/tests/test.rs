use ara_hangul::*;

mod check_functions {
	use super::*;

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

		#[test]
		fn must_include_complete_hangul() {
			assert!(Hangul::is_hangul('가'));
			assert!(Hangul::is_hangul('각'));
			assert!(Hangul::is_hangul('각'));
			assert!(Hangul::is_hangul('힣'));
		}

		#[test]
		fn must_include_partial_characters() {
			assert!(Hangul::is_hangul('ㄱ'));
			assert!(Hangul::is_hangul('ㅏ'));
			assert!(Hangul::is_hangul('ㄿ'));
		}

		#[test]
		fn must_ignore_old_hangul_characters() {
			assert!(!Hangul::is_hangul('ㅹ'));
			assert!(!Hangul::is_hangul('ᄭ'));
		}

		#[test]
		fn must_ignore_hcf() {
			assert!(!Hangul::is_hangul('\u{115F}'));
		}
	}

	mod is_jamo {
		use super::*;

		#[test]
		fn must_include_consonants() {
			assert!(Jamo::is_jamo('ㅍ'));
		}

		#[test]
		fn must_include_double_consonants() {
			assert!(Jamo::is_jamo('ㅉ'));
		}

		#[test]
		fn must_include_double_final_consonants() {
			assert!(Jamo::is_jamo('ㅀ'))
		}

		#[test]
		fn must_include_vowels() {
			assert!(Jamo::is_jamo('ㅗ'));
		}

		#[test]
		fn must_include_diphthong() {
			assert!(Jamo::is_jamo('ㅞ'));
		}
	}

	mod is_choseong {
		use super::*;

		#[test]
		fn must_include_consonants() {
			assert!(Choseong::is_choseong('ㄴ'));
		}

		#[test]
		fn must_include_double_consonants() {
			assert!(Choseong::is_choseong('ㅆ'));
		}

		#[test]
		fn must_not_include_double_final_consonants() {
			assert!(!Choseong::is_choseong('ㄵ'))
		}
	}

	mod is_jungseong {
		use super::*;

		#[test]
		fn all_vowels_must_to_be_a_valid_jungseong() {
			assert!(Jungseong::is_jungseong('ㅖ'))
		}
	}
}
