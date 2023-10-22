use lingua::Language;
use pyo3::prelude::*;

#[pyclass(name = "Language", frozen)]
#[derive(Clone, Copy, Debug)]
pub enum PyLanguage {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Maori,
    Marathi,
    Mongolian,
    Nynorsk,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Xhosa,
    Yoruba,
    Zulu,
}
impl PyLanguage {
    pub fn clone_from_py(slf: &Py<Self>) -> Self {
        slf.get().clone()
    }
}
impl From<Language> for PyLanguage {
    fn from(language: Language) -> Self {
        match language {
            Language::Afrikaans => PyLanguage::Afrikaans,
            Language::Albanian => PyLanguage::Albanian,
            Language::Arabic => PyLanguage::Arabic,
            Language::Armenian => PyLanguage::Armenian,
            Language::Azerbaijani => PyLanguage::Azerbaijani,
            Language::Basque => PyLanguage::Basque,
            Language::Belarusian => PyLanguage::Belarusian,
            Language::Bengali => PyLanguage::Bengali,
            Language::Bokmal => PyLanguage::Bokmal,
            Language::Bosnian => PyLanguage::Bosnian,
            Language::Bulgarian => PyLanguage::Bulgarian,
            Language::Catalan => PyLanguage::Catalan,
            Language::Chinese => PyLanguage::Chinese,
            Language::Croatian => PyLanguage::Croatian,
            Language::Czech => PyLanguage::Czech,
            Language::Danish => PyLanguage::Danish,
            Language::Dutch => PyLanguage::Dutch,
            Language::English => PyLanguage::English,
            Language::Esperanto => PyLanguage::Esperanto,
            Language::Estonian => PyLanguage::Estonian,
            Language::Finnish => PyLanguage::Finnish,
            Language::French => PyLanguage::French,
            Language::Ganda => PyLanguage::Ganda,
            Language::Georgian => PyLanguage::Georgian,
            Language::German => PyLanguage::German,
            Language::Greek => PyLanguage::Greek,
            Language::Gujarati => PyLanguage::Gujarati,
            Language::Hebrew => PyLanguage::Hebrew,
            Language::Hindi => PyLanguage::Hindi,
            Language::Hungarian => PyLanguage::Hungarian,
            Language::Icelandic => PyLanguage::Icelandic,
            Language::Indonesian => PyLanguage::Indonesian,
            Language::Irish => PyLanguage::Irish,
            Language::Italian => PyLanguage::Italian,
            Language::Japanese => PyLanguage::Japanese,
            Language::Kazakh => PyLanguage::Kazakh,
            Language::Korean => PyLanguage::Korean,
            Language::Latin => PyLanguage::Latin,
            Language::Latvian => PyLanguage::Latvian,
            Language::Lithuanian => PyLanguage::Lithuanian,
            Language::Macedonian => PyLanguage::Macedonian,
            Language::Malay => PyLanguage::Malay,
            Language::Maori => PyLanguage::Maori,
            Language::Marathi => PyLanguage::Marathi,
            Language::Mongolian => PyLanguage::Mongolian,
            Language::Nynorsk => PyLanguage::Nynorsk,
            Language::Persian => PyLanguage::Persian,
            Language::Polish => PyLanguage::Polish,
            Language::Portuguese => PyLanguage::Portuguese,
            Language::Punjabi => PyLanguage::Punjabi,
            Language::Romanian => PyLanguage::Romanian,
            Language::Russian => PyLanguage::Russian,
            Language::Serbian => PyLanguage::Serbian,
            Language::Shona => PyLanguage::Shona,
            Language::Slovak => PyLanguage::Slovak,
            Language::Slovene => PyLanguage::Slovene,
            Language::Somali => PyLanguage::Somali,
            Language::Sotho => PyLanguage::Sotho,
            Language::Spanish => PyLanguage::Spanish,
            Language::Swahili => PyLanguage::Swahili,
            Language::Swedish => PyLanguage::Swedish,
            Language::Tagalog => PyLanguage::Tagalog,
            Language::Tamil => PyLanguage::Tamil,
            Language::Telugu => PyLanguage::Telugu,
            Language::Thai => PyLanguage::Thai,
            Language::Tsonga => PyLanguage::Tsonga,
            Language::Tswana => PyLanguage::Tswana,
            Language::Turkish => PyLanguage::Turkish,
            Language::Ukrainian => PyLanguage::Ukrainian,
            Language::Urdu => PyLanguage::Urdu,
            Language::Vietnamese => PyLanguage::Vietnamese,
            Language::Welsh => PyLanguage::Welsh,
            Language::Xhosa => PyLanguage::Xhosa,
            Language::Yoruba => PyLanguage::Yoruba,
            Language::Zulu => PyLanguage::Zulu,
        }
    }
}
impl Into<Language> for PyLanguage {
    fn into(self) -> Language {
        match self {
            PyLanguage::Afrikaans => Language::Afrikaans,
            PyLanguage::Albanian => Language::Albanian,
            PyLanguage::Arabic => Language::Arabic,
            PyLanguage::Armenian => Language::Armenian,
            PyLanguage::Azerbaijani => Language::Azerbaijani,
            PyLanguage::Basque => Language::Basque,
            PyLanguage::Belarusian => Language::Belarusian,
            PyLanguage::Bengali => Language::Bengali,
            PyLanguage::Bokmal => Language::Bokmal,
            PyLanguage::Bosnian => Language::Bosnian,
            PyLanguage::Bulgarian => Language::Bulgarian,
            PyLanguage::Catalan => Language::Catalan,
            PyLanguage::Chinese => Language::Chinese,
            PyLanguage::Croatian => Language::Croatian,
            PyLanguage::Czech => Language::Czech,
            PyLanguage::Danish => Language::Danish,
            PyLanguage::Dutch => Language::Dutch,
            PyLanguage::English => Language::English,
            PyLanguage::Esperanto => Language::Esperanto,
            PyLanguage::Estonian => Language::Estonian,
            PyLanguage::Finnish => Language::Finnish,
            PyLanguage::French => Language::French,
            PyLanguage::Ganda => Language::Ganda,
            PyLanguage::Georgian => Language::Georgian,
            PyLanguage::German => Language::German,
            PyLanguage::Greek => Language::Greek,
            PyLanguage::Gujarati => Language::Gujarati,
            PyLanguage::Hebrew => Language::Hebrew,
            PyLanguage::Hindi => Language::Hindi,
            PyLanguage::Hungarian => Language::Hungarian,
            PyLanguage::Icelandic => Language::Icelandic,
            PyLanguage::Indonesian => Language::Indonesian,
            PyLanguage::Irish => Language::Irish,
            PyLanguage::Italian => Language::Italian,
            PyLanguage::Japanese => Language::Japanese,
            PyLanguage::Kazakh => Language::Kazakh,
            PyLanguage::Korean => Language::Korean,
            PyLanguage::Latin => Language::Latin,
            PyLanguage::Latvian => Language::Latvian,
            PyLanguage::Lithuanian => Language::Lithuanian,
            PyLanguage::Macedonian => Language::Macedonian,
            PyLanguage::Malay => Language::Malay,
            PyLanguage::Maori => Language::Maori,
            PyLanguage::Marathi => Language::Marathi,
            PyLanguage::Mongolian => Language::Mongolian,
            PyLanguage::Nynorsk => Language::Nynorsk,
            PyLanguage::Persian => Language::Persian,
            PyLanguage::Polish => Language::Polish,
            PyLanguage::Portuguese => Language::Portuguese,
            PyLanguage::Punjabi => Language::Punjabi,
            PyLanguage::Romanian => Language::Romanian,
            PyLanguage::Russian => Language::Russian,
            PyLanguage::Serbian => Language::Serbian,
            PyLanguage::Shona => Language::Shona,
            PyLanguage::Slovak => Language::Slovak,
            PyLanguage::Slovene => Language::Slovene,
            PyLanguage::Somali => Language::Somali,
            PyLanguage::Sotho => Language::Sotho,
            PyLanguage::Spanish => Language::Spanish,
            PyLanguage::Swahili => Language::Swahili,
            PyLanguage::Swedish => Language::Swedish,
            PyLanguage::Tagalog => Language::Tagalog,
            PyLanguage::Tamil => Language::Tamil,
            PyLanguage::Telugu => Language::Telugu,
            PyLanguage::Thai => Language::Thai,
            PyLanguage::Tsonga => Language::Tsonga,
            PyLanguage::Tswana => Language::Tswana,
            PyLanguage::Turkish => Language::Turkish,
            PyLanguage::Ukrainian => Language::Ukrainian,
            PyLanguage::Urdu => Language::Urdu,
            PyLanguage::Vietnamese => Language::Vietnamese,
            PyLanguage::Welsh => Language::Welsh,
            PyLanguage::Xhosa => Language::Xhosa,
            PyLanguage::Yoruba => Language::Yoruba,
            PyLanguage::Zulu => Language::Zulu,
        }
    }
}
