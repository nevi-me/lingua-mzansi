use lingua::Language::{Afrikaans, English, Sotho, Tsonga, Tswana, Xhosa, Zulu};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

fn main() {
    let languages = vec![Tswana, English, Afrikaans, Sotho, Zulu, Tsonga, Xhosa];
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();

    // test 1
    let detected_language: Option<Language> = detector.detect_language_of("Kgosi ya rona e botlhale thata");
    println!("The detected language is {:?}", detected_language);
    assert_eq!(detected_language, Some(Tswana));

    // test 2
    let detected_language: Option<Language> = detector.detect_language_of("Morena oa rona o bohlale haholo");
    println!("The detected language is {:?}", detected_language);
    assert_eq!(detected_language, Some(Sotho));

    // test 3
    let detected_language: Option<Language> = detector.detect_language_of("Ons koning is baie wys");
    println!("The detected language is {:?}", detected_language);
    assert_eq!(detected_language, Some(Afrikaans));

    // test 4
    let detected_language: Option<Language> = detector.detect_language_of("Ukumkani wethu ulumke kakhulu");
    println!("The detected language is {:?}", detected_language);
    assert_eq!(detected_language, Some(Xhosa));

    // test 5
    let detected_language: Option<Language> = detector.detect_language_of("Inkosi yethu ihlakaniphe kakhulu");
    println!("The detected language is {:?}", detected_language);
    assert_eq!(detected_language, Some(Zulu));
}
