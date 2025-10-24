use std::{fs::OpenOptions, io::{BufWriter, Write}};

use rand::{seq::SliceRandom, thread_rng};

pub fn random_sentence() -> String {
    let subjects = [
        "The curious fox", "A sleepy robot", "My neighbor", "This library",
        "The tiny server", "An ancient modem", "Our compiler", "A brave kernel",
    ];
    let verbs = [
        "optimizes", "deploys", "refactors", "reboots", "indexes",
        "parses", "streams", "caches",
    ];
    let objects = [
        "rusty crates", "mysterious packets", "latent bugs", "unexpected inputs",
        "test fixtures", "TLS handshakes", "unicode graphemes", "log files",
    ];
    let adverbs = [
        "silently", "boldly", "reluctantly", "gracefully",
        "sporadically", "happily", "curiously", "efficiently",
    ];

    let mut rng = thread_rng();
    let s = subjects.choose(&mut rng).unwrap();
    let v = verbs.choose(&mut rng).unwrap();
    let o = objects.choose(&mut rng).unwrap();
    let a = adverbs.choose(&mut rng).unwrap();

    let mut sentence = format!("{s} {v} {o} {a}");
    if !sentence.ends_with('.') { sentence.push('.'); }
    sentence
}

// Optional: generate N sentences
pub fn random_sentences(n: usize) -> Vec<String> {
    (0..n).map(|_| random_sentence()).collect()
}

pub fn append_log_entry(file_path: &str, message: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", message)?;
    writer.flush()
}