* Current todos
- [ ] Safety safety safety - the Perceptron tagger currently makes several assumptions about the
input being non-empty (everywhere that `.unwrap()` or a straight index is used)
- [ ] Use failure and the `Result` type
- [ ] Don't clone things as much; use the following pattern instead of entry:

```rust
scores.get_mut(label)
    .map(|w| *w += (*weight as f64 * val) as f64)
    .unwrap_or_else(|| { scores.insert(label.to_owned(), (*weight as f64 * val) as f64); });
```