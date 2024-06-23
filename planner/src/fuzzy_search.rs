use deunicode::deunicode;
use itertools::Itertools;
use num_traits::identities::Zero;
use ordered_float::NotNan;
use strsim::normalized_damerau_levenshtein;

#[derive(Clone)]
pub struct Term<'a> {
    value: &'a str,
    weight: f32,
}

impl<'a> Term<'a> {
    pub fn new(value: &'a str, weight: f32) -> Term<'a> {
        Self { value, weight }
    }
}

pub trait Searchable {
    fn search_terms(&self) -> Vec<Term>;
}

impl Searchable for String {
    fn search_terms(&self) -> Vec<Term> {
        vec![Term::new(self, 1.0)]
    }
}

impl Searchable for &str {
    fn search_terms(&self) -> Vec<Term> {
        vec![Term::new(self, 1.0)]
    }
}

impl Searchable for Vec<Term<'_>> {
    fn search_terms(&self) -> Vec<Term<'_>> {
        (*self).clone()
    }
}

fn normalize(s: &str) -> String {
    deunicode(s).to_ascii_lowercase()
}

pub fn fuzzy_search<'a, T, Iter>(query: impl AsRef<str>, iter: Iter) -> impl Iterator<Item = T>
where
    Iter: Iterator<Item = T>,
    T: Searchable + 'a,
{
    let query = query.as_ref();
    iter.map(|value| (fuzzy_score(query, &value), value))
        .filter(|(score, _)| score.into_inner() > 0.3)
        .sorted_by_key(|(score, _)| NotNan::zero() - score)
        .map(|(_score, value)| value)
}

pub fn fuzzy_score<T>(query: impl AsRef<str>, value: &T) -> NotNan<f32>
where
    T: Searchable,
{
    let query = normalize(query.as_ref());
    value
        .search_terms()
        .into_iter()
        .map(|term| {
            let normalized_value = &normalize(term.value);
            let mut score = normalized_damerau_levenshtein(query.as_ref(), normalized_value) as f32;
            if !query.is_empty() && normalized_value.starts_with(&query) {
                score += 0.5;
            } else if !query.is_empty() && normalized_value.contains(&query) {
                score += 0.25;
            }
            NotNan::<f32>::new(score * term.weight).unwrap()
        })
        .max()
        .unwrap_or(NotNan::zero())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_search_priority() {
        let words = vec![
            "foo", "bar", "baz", "foobar", "foobaz", "barbaz", "bazbaz", "barfoo", "berfooda",
            "fto",
        ];
        let query = "foo";
        let expected = vec!["foo", "foobar", "foobaz", "barfoo", "fto", "berfooda"];
        let actual = fuzzy_search(query, words.clone().into_iter()).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn fuzzy_score_evaluation() {
        assert_eq!(fuzzy_score("foo", &"foo"), NotNan::new(1.5).unwrap());
        assert_eq!(fuzzy_score("foo", &"foobar"), NotNan::new(1.0).unwrap());
        assert_eq!(fuzzy_score("foo", &"foobaz"), NotNan::new(1.0).unwrap());
        assert_eq!(fuzzy_score("foo", &"barfoo"), NotNan::new(0.75).unwrap());
        assert_eq!(fuzzy_score("foo", &"fto"), NotNan::new(0.6666667).unwrap());
        assert_eq!(fuzzy_score("foo", &"berfooda"), NotNan::new(0.625).unwrap());
        assert_eq!(fuzzy_score("foo", &"bar"), NotNan::new(0.0).unwrap());
        assert_eq!(fuzzy_score("foo", &"baz"), NotNan::new(0.0).unwrap());
        assert_eq!(fuzzy_score("foo", &"barbaz"), NotNan::new(0.0).unwrap());
        assert_eq!(fuzzy_score("foo", &"bazbaz"), NotNan::new(0.0).unwrap());
    }

    #[test]
    fn fuzzy_score_with_weights() {
        let terms = vec![Term::new("where", 1.0), Term::new("foo", 0.5)];
        assert_eq!(fuzzy_score("where", &terms), NotNan::new(1.5).unwrap());
        assert_eq!(fuzzy_score("foo", &terms), NotNan::new(0.75).unwrap());
        assert_eq!(
            fuzzy_score("where is the foo", &terms),
            NotNan::new(0.3125).unwrap()
        );
        assert_eq!(
            fuzzy_score("is the", &terms),
            NotNan::new(0.16666667).unwrap()
        );
    }

    #[test]
    fn normalize_russian() {
        assert_eq!(normalize("Ревущий фьорд"), "revushchii f'ord");
        assert_eq!(normalize("Пиратская Бухта"), "piratskaia bukhta");
    }

    #[test]
    fn normalize_french() {
        assert_eq!(normalize("Confrérie du Thorium"), "confrerie du thorium");
    }

    #[test]
    fn normalize_german() {
        assert_eq!(normalize("Festung der Stürme"), "festung der sturme");
    }

    #[test]
    fn normalize_korean() {
        assert_eq!(normalize("아즈샤라"), "ajeusyara");
        assert_eq!(normalize("하이잘"), "haijal");
    }

    #[test]
    fn normalize_chinese() {
        assert_eq!(normalize("夜空之歌"), "ye kong zhi ge");
        assert_eq!(normalize("雲蛟衛"), "yun jiao wei");
    }
}
