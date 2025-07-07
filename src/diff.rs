use imara_diff::{Algorithm, BasicLineDiffPrinter, Diff, InternedInput, UnifiedDiffConfig};

pub fn diff(before: &str, after: &str) -> String {
    let input = InternedInput::new(before, after);

    Diff::compute(Algorithm::Histogram, &input)
        .unified_diff(
            &BasicLineDiffPrinter(&input.interner),
            UnifiedDiffConfig::default(),
            &input,
        )
        .to_string()
}
