//! An averaged perceptron part of speech tagger

use super::*;

use std::collections::{HashMap, HashSet};

pub struct AveragedPerceptron {
    classes: HashSet<String>,
    instances: usize,
    stamps: HashMap<String, f64>,
    tags: HashMap<String, String>,
    totals: HashMap<String, f64>,
    weights: HashMap<String, HashMap<String, f64>>,
}

impl AveragedPerceptron {
    pub fn new(
        weights: HashMap<String, HashMap<String, f64>>,
        tags: HashMap<String, String>,
        classes: HashSet<String>,
    ) -> AveragedPerceptron {
        AveragedPerceptron {
            classes: classes,
            instances: 0,
            stamps: HashMap::new(),
            tags: tags,
            totals: HashMap::new(),
            weights: weights,
        }
    }

    pub fn predict(&self, features: HashMap<String, f64>) -> String {
        let mut scores: HashMap<String, f64> = HashMap::new();
        for (feat, val) in features {
            if self.weights.get(&feat).is_none() || val == 0.0 {
                continue;
            }
            let weights = &self.weights[&feat];
            for (label, weight) in weights {
                *scores.entry(label.clone()).or_insert(0.0) += (*weight as f64 * val) as f64;
            }
        }

        self.classes
            .iter()
            .map(|i| (scores[i] * 100000.0) as usize)
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
                let total = self.totals.entry(key.clone()).or_insert(0.0);
                *total += (self.instances as f64 - self.stamps[&key]) * weight;
                let averaged = (*total / (self.instances as f64) * 1000.0).round() / 1000.0;
                if averaged != 0.0 {
                    new.insert(class.clone(), averaged);
                }
            }
            *weights = new;
        }
    }

    fn update_feat(&mut self, c: &str, f: &str, v: f64, w: f64) {
        let key = format!("{}-{}", c, f);
        *self.totals.entry(key.clone()).or_insert(0.0) =
            (self.instances as f64 - self.stamps[&key]) * w;
        *self.stamps.entry(key.clone()).or_insert(0.0) = self.instances as f64;
        *self.weights
            .entry(key.clone())
            .or_insert_with(HashMap::new)
            .entry(c.to_owned())
            .or_insert(0.0) = w + v;
    }
}

struct PerceptronTagger {
    tags: HashMap<String, String>,
    model: AveragedPerceptron,
}

impl PerceptronTagger {}
