use std::collections::HashSet;
use std::vec::Vec;
use tiktoken_rs::tokenizer::Tokenizer;

// This converts the available tokens to strings that Elixir can understand.
// https://github.com/zurawiki/tiktoken-rs/blob/main/tiktoken-rs/src/tokenizer.rs#L22
fn token_string(token: Tokenizer) -> &'static str {
    match token {
        Tokenizer::Cl100kBase => "Cl100kBase",
        Tokenizer::P50kBase => "P50kBase",
        Tokenizer::R50kBase => "R50kBase",
        Tokenizer::P50kEdit => "P50kEdit",
        Tokenizer::Gpt2 => "Gpt2",
    }
}

#[rustler::nif]
fn get_tokenizer(model: &str) -> Option<&str> {
    let tokenizer: Tokenizer = tiktoken_rs::tokenizer::get_tokenizer(model).unwrap();
    Some(token_string(tokenizer))
}

// p50k

#[rustler::nif]
fn p50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn p50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn p50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn p50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::p50k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// p50k edit

#[rustler::nif]
fn p50k_edit_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn p50k_edit_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn p50k_edit_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn p50k_edit_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::p50k_edit_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// r50k

#[rustler::nif]
fn r50k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn r50k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn r50k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn r50k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::r50k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

// cl100k

#[rustler::nif]
fn cl100k_encode_ordinary(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_ordinary(text))
    }
}

#[rustler::nif]
fn cl100k_encode(text: &str, allowed_special: Vec<&str>) -> Result<Vec<usize>, String> {
    let set = HashSet::from_iter(allowed_special.iter().cloned());
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode(text, set))
    }
}

#[rustler::nif]
fn cl100k_encode_with_special_tokens(text: &str) -> Result<Vec<usize>, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        Ok(guard.encode_with_special_tokens(text))
    }
}

#[rustler::nif]
fn cl100k_decode(ids: Vec<usize>) -> Result<String, String> {
    let bpe = tiktoken_rs::cl100k_base_singleton();
    {
        let guard = bpe.lock();
        match guard.decode(ids) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}

rustler::init!(
    "Elixir.ExTiktoken.Native",
    [
        get_tokenizer,
        p50k_encode_ordinary,
        p50k_encode,
        p50k_encode_with_special_tokens,
        p50k_decode,
        p50k_edit_encode_ordinary,
        p50k_edit_encode,
        p50k_edit_encode_with_special_tokens,
        p50k_edit_decode,
        r50k_encode_ordinary,
        r50k_encode,
        r50k_encode_with_special_tokens,
        r50k_decode,
        cl100k_encode_ordinary,
        cl100k_encode,
        cl100k_encode_with_special_tokens,
        cl100k_decode
    ]
);
