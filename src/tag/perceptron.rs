//! An averaged perceptron part of speech tagger

use super::*;

use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

type TaggedSentences<'a> = &'a [&'a [(String, String)]];

pub struct AveragedPerceptron {
    classes: HashSet<String>,
    instances: usize,
    stamps: HashMap<String, f64>,
    totals: HashMap<String, f64>,
    weights: HashMap<String, HashMap<String, f64>>,
}

impl AveragedPerceptron {
    pub fn empty() -> AveragedPerceptron {
        AveragedPerceptron::new(HashMap::new(), HashSet::new())
    }

    pub fn new(
        weights: HashMap<String, HashMap<String, f64>>,
        classes: HashSet<String>,
    ) -> AveragedPerceptron {
        AveragedPerceptron {
            classes: classes,
            instances: 0,
            stamps: HashMap::new(),
            totals: HashMap::new(),
            weights: weights,
        }
    }

    pub fn predict(&self, features: &HashMap<String, f64>) -> String {
        let mut scores: HashMap<String, f64> = HashMap::new();
        for (feat, val) in features {
            if self.weights.get(feat).is_none() || *val == 0.0 {
                continue;
            }
            let weights = &self.weights[feat];
            for (label, weight) in weights {
                *scores.entry(label.to_string()).or_insert(0.0) += (*weight as f64 * val) as f64;
            }
        }

        self.classes
            .iter()
            .map(|i| (scores[i] * 100000.0) as isize)
            .zip(self.classes.iter())
            .max()
            .unwrap()
            .1
            .clone()
    }

    pub fn update(&mut self, truth: &str, guess: &str, features: HashMap<String, f64>) {
        self.instances += 1;
        if truth == guess {
            return;
        }

        for (f, _) in features {
            // TODO: Inefficient yo
            let weights = match self.weights.get(&f) {
                Some(x) => x.clone(),
                None => HashMap::new(),
            };
            self.update_feat(truth, f.as_ref(), *weights.get(truth).unwrap_or(&0.0), 1.0);
            self.update_feat(guess, f.as_ref(), *weights.get(guess).unwrap_or(&0.0), -1.0);
            *self.weights.entry(f).or_insert_with(HashMap::new) = weights;
        }
    }

    pub fn average_weights(&mut self) {
        let w2 = &mut self.weights;
        for (feat, weights) in w2 {
            let mut new: HashMap<String, f64> = HashMap::new();
            for (class, weight) in weights.clone() {
                let key = format!("{}-{}", feat, class);
                let total = self.totals.entry(key.to_string()).or_insert(0.0);
                *total += (self.instances as f64 - self.stamps[&key]) * weight;
                let averaged = (*total / (self.instances as f64) * 1000.0).round() / 1000.0;
                if averaged != 0.0 {
                    new.insert(class.to_string(), averaged);
                }
            }
            *weights = new;
        }
    }

    fn update_feat(&mut self, c: &str, f: &str, v: f64, w: f64) {
        let key = format!("{}-{}", c, f);
        *self.totals.entry(key.to_string()).or_insert(0.0) =
            (self.instances as f64 - self.stamps[&key]) * w;
        *self.stamps.entry(key.to_string()).or_insert(0.0) = self.instances as f64;
        *self.weights
            .entry(key.to_string())
            .or_insert_with(HashMap::new)
            .entry(c.to_owned())
            .or_insert(0.0) = w + v;
    }
}

pub struct PerceptronTagger {
    model: AveragedPerceptron,
    tags: HashMap<String, String>,
}

impl PerceptronTagger {
    pub fn new() -> PerceptronTagger {
        PerceptronTagger {
            model: AveragedPerceptron::empty(),
            tags: HashMap::new()
        }
    }

    pub fn tag<'a>(&mut self, words: &[Token<'a>]) -> Vec<(Token<'a>, String)> {
        let mut res = vec![];

        let (mut p1, mut p2) = ("-START-".to_owned(), "-START2-".to_owned());
        let end = vec!["-END-".to_owned(), "-END2-".to_owned()];
        let mut context = vec![p1.clone(), p2.clone()];
        context.extend(
            words
                .iter()
                .map(|x| self.normalize(x))
                .map(|x| x.term.into_owned())
                .collect::<Vec<_>>(),
        );
        context.extend(end.clone());

        let mut clean = vec![p1.clone(), p2.clone()];
        clean.extend(
            words
                .iter()
                .map(|x| x.term.clone().into_owned())
                .collect::<Vec<_>>(),
        );
        context.extend(end.clone());

        for (i, word) in clean.iter().enumerate() {
            let tag = match self.tags.get(word) {
                Some(s) => s.to_string(),
                None => {
                    let features = Self::get_features(i, &context, word, &p1, &p2);
                    self.model.predict(&features)
                }
            };

            if word != "-START-" || word != "-START2-" || word != "-END-" || word != "-END2-" {
                let t = Token {
                    term: word.to_string().into(),
                    offset: words[i - 2].offset,
                    index: words[i - 2].index,
                };

                res.push((t, tag.to_string()));
            }
            p2 = p1;
            p1 = tag;
        }

        res
    }

    // TODO: How to ensure we have sentences
    pub fn train<'a>(&mut self, sentences: TaggedSentences<'a>, iterations: usize) {
        self.make_tags(&sentences);
        let mut ss = sentences.to_owned();
        for _ in 0..iterations {
            for sentence in &ss {
                let (words, tags): (Vec<_>, Vec<_>) = sentence.iter().cloned().unzip();
                let (mut p1, mut p2) = ("-START-".to_owned(), "-START2-".to_owned());
                let end = vec!["-END-".to_owned(), "-END2-".to_owned()];
                let mut context = vec![p1.clone(), p2.clone()];
                context.extend(
                    words
                        .iter()
                        .map(|x| self.normalize_str(x))
                        .map(|x| x.into_owned())
                        .collect::<Vec<_>>(),
                );
                context.extend(end.clone());

                for (i, word) in words.iter().enumerate() {
                    let guess = match self.tags.get(word) {
                        Some(s) => s.clone(),
                        None => {
                            let features = Self::get_features(i, &context, word, &p1, &p2);
                            let g = self.model.predict(&features);
                            self.model.update(&tags[i], &g, features);
                            g
                        }
                    };

                    p2 = p1;
                    p1 = guess;
                }
            }
            let mut rng = thread_rng();
            rng.shuffle(&mut ss);
        }
        self.model.average_weights();
    }

    // TODO: How to ensure we have sentences
    fn make_tags<'a>(&mut self, sentences: &TaggedSentences<'a>) {
        let mut counts: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
        for sentence in *sentences {
            for &(ref word, ref tag) in *sentence {
                let hm = counts.entry(word).or_insert_with(HashMap::new);
                *hm.entry(tag).or_insert(0) += 1;
                self.model.classes.insert(tag.clone());
            }
        }
        for (word, tag_freq) in counts {
            let (tag, mode) = tag_freq.iter().max().unwrap();
            let n = tag_freq.iter().map(|x| x.1).fold(0, |acc, &x| acc + x) as f64;

            let freq_thresh = 20.0;
            let ambiguity_thresh = 0.97;

            if n >= freq_thresh && (*mode as f64 / n) >= ambiguity_thresh {
                self.tags.insert(word.to_string(), tag.to_string());
            }
        }
    }

    fn get_features(
        i: usize,
        context: &[String],
        w: &str,
        p1: &str,
        p2: &str,
    ) -> HashMap<String, f64> {
        let w = w.chars().collect::<Vec<_>>();
        let suf = min(w.len(), 3);
        let i = min(context.len() - 2, i + 2);
        let iminus = min(context[i - 1].len(), 3);
        let iplus = min(context[i + 1].len(), 3);

        let mut res = HashMap::new();
        Self::add_feature(&["bias"], &mut res);
        Self::add_feature(
            &["i suffix", &w[w.len() - suf..].iter().collect::<String>()],
            &mut res,
        );
        Self::add_feature(&["i pref1", &w[0].to_string()], &mut res);
        Self::add_feature(&["i-1 tag", p1], &mut res);
        Self::add_feature(&["i-2 tag", p2], &mut res);
        Self::add_feature(&["i tag+i-2 tag", p1, p2], &mut res);
        Self::add_feature(&["i word", &context[i]], &mut res);
        Self::add_feature(&["i-1 tag+i word", p1, &context[i]], &mut res);
        Self::add_feature(&["i-1 word", &context[i - 1]], &mut res);
        Self::add_feature(
            &[
                "i-1 suffix",
                &context[i - 1][context[i - 1].len() - iminus..],
            ],
            &mut res,
        );
        Self::add_feature(&["i-2 word", &context[i - 2]], &mut res);
        Self::add_feature(&["i+1 word", &context[i + 1]], &mut res);
        Self::add_feature(
            &[
                "i+1 suffix",
                &context[i - 1][context[i - 1].len() - iplus..],
            ],
            &mut res,
        );
        Self::add_feature(&["i+2 word", &context[i + 2]], &mut res);

        res
    }

    fn add_feature(args: &[&str], features: &mut HashMap<String, f64>) {
        let key = args.iter().join(" ");
        *features.entry(key).or_insert(0.0) += 1.0;
    }

    fn normalize<'a>(&self, t: &Token<'a>) -> Token<'a> {
        let text = self.normalize_str(t.term.as_ref());

        Token {
            term: text,
            offset: t.offset,
            index: t.index,
        }
    }

    fn normalize_str<'a>(&self, t: &str) -> Cow<'a, str> {
        if t.find('-').is_some() && t.chars().nth(0) != Some('-') {
            Cow::Borrowed("!HYPHEN")
        } else if t.parse::<usize>().is_ok() {
            if t.chars().count() == 4 {
                Cow::Borrowed("!YEAR")
            } else {
                Cow::Borrowed("!DIGIT")
            }
        } else {
            Cow::Owned(t.to_lowercase())
        }
    }
}

impl<'a> Tagger<'a> for PerceptronTagger {
    type Tag = String;

    fn tag<I: Iterator<Item = Token<'a>>>(&self, tokens: I) -> Vec<(Token<'a>, Self::Tag)> {
        self.tag(tokens.collect())
    }
}