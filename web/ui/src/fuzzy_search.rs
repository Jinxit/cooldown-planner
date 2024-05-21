use deunicode::deunicode;
use itertools::Itertools;
use num_traits::identities::Zero;
use ordered_float::NotNan;
use strsim::normalized_damerau_levenshtein;

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
