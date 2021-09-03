mod decoder;

fn main() {
    let encoder = Encoder;
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness
        .ignore([
            "valid/array/table-array-string-backslash.toml",
            "valid/comment/tricky.toml",
            "valid/float/exponent.toml",
            "valid/float/inf-and-nan.toml",
            "valid/float/underscore.toml",
            "valid/float/zero.toml",
            "valid/key/escapes.toml",
            "valid/string/double-quote-escape.toml",
            "valid/string/escape-tricky.toml",
            "valid/string/escapes.toml",
            "valid/string/nl.toml",
            "valid/string/raw.toml",
            // Can't verify until decoder is fixed
            "valid/string/multiline-quotes.toml",
        ])
        .unwrap();
    harness.test();
}

pub struct Encoder;

impl toml_test_harness::Encoder for Encoder {
    fn name(&self) -> &str {
        "toml_edit"
    }

    fn encode(&self, data: toml_test_harness::Decoded) -> Result<String, toml_test_harness::Error> {
        let doc = decoded_to_document(&data)?;
        Ok(doc.to_string())
    }
}

fn decoded_to_document(
    decoded: &toml_test_harness::Decoded,
) -> Result<toml_edit::Document, toml_test_harness::Error> {
    let item = toml_edit::Item::Table(root_from_decoded(decoded)?);
    let mut doc = toml_edit::Document::new();
    doc.root = item;
    Ok(doc)
}

fn root_from_decoded(
    decoded: &toml_test_harness::Decoded,
) -> Result<toml_edit::Table, toml_test_harness::Error> {
    match decoded {
        toml_test_harness::Decoded::Value(_) => {
            Err(toml_test_harness::Error::new("Root cannot be a value"))
        }
        toml_test_harness::Decoded::Table(value) => value
            .iter()
            .map(|(k, v)| {
                let k = k.as_str();
                let v = from_decoded(v)?;
                Ok((k, v))
            })
            .collect(),
        toml_test_harness::Decoded::Array(_) => {
            Err(toml_test_harness::Error::new("Root cannot be an array"))
        }
    }
}

fn from_decoded(
    decoded: &toml_test_harness::Decoded,
) -> Result<toml_edit::Value, toml_test_harness::Error> {
    let value = match decoded {
        toml_test_harness::Decoded::Value(value) => from_decoded_value(value)?,
        toml_test_harness::Decoded::Table(value) => {
            toml_edit::Value::InlineTable(from_table(value)?)
        }
        toml_test_harness::Decoded::Array(value) => toml_edit::Value::Array(from_array(value)?),
    };
    Ok(value)
}

fn from_decoded_value(
    decoded: &toml_test_harness::DecodedValue,
) -> Result<toml_edit::Value, toml_test_harness::Error> {
    let value: toml_edit::Value = match decoded {
        toml_test_harness::DecodedValue::String(value) => value.into(),
        toml_test_harness::DecodedValue::Integer(value) => value
            .parse::<i64>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::Float(value) => value
            .parse::<f64>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::Bool(value) => value
            .parse::<bool>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::Datetime(value) => value
            .parse::<toml_edit::OffsetDateTime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::DatetimeLocal(value) => value
            .parse::<toml_edit::LocalDateTime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::DateLocal(value) => value
            .parse::<toml_edit::LocalDate>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedValue::TimeLocal(value) => value
            .parse::<toml_edit::LocalTime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
    };
    Ok(value)
}

fn from_table(
    decoded: &std::collections::HashMap<String, toml_test_harness::Decoded>,
) -> Result<toml_edit::InlineTable, toml_test_harness::Error> {
    decoded
        .iter()
        .map(|(k, v)| {
            let v = from_decoded(v)?;
            Ok((k, v))
        })
        .collect()
}

fn from_array(
    decoded: &[toml_test_harness::Decoded],
) -> Result<toml_edit::Array, toml_test_harness::Error> {
    decoded.iter().map(|v| from_decoded(v)).collect()
}