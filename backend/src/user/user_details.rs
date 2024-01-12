use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct UserDetails {
    email: String,
    name: String,
}

struct UserDetailsVisitor;

// impl<'de> de::Visitor<'de> for UserDetailsVisitor {
//     type Value = AttributeValue;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("An enum")
//     }
//
//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         let idx = v.find("(").unwrap_or(1);
//         let substr = &v[0..idx];
//
//         match substr {
//             "S" => Ok(AttributeValue::S("".to_string())),
//         }
//     }
// }
