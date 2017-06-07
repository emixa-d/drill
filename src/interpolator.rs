use std::collections::HashMap;

use regex::{Regex, Captures};
use serde_json::Value;
use colored::*;
use yaml_rust::Yaml;

pub struct Interpolator<'a> {
  base_url: &'a String,
  context: &'a HashMap<String, Yaml>,
  responses: &'a HashMap<String, Value>,
}

impl<'a> Interpolator<'a> {
  pub fn new(base_url: &'a String, context: &'a HashMap<String, Yaml>, responses: &'a HashMap<String, Value>) -> Interpolator<'a> {
    Interpolator {
      base_url: base_url,
      context: context,
      responses: responses
    }
  }

  pub fn resolve(&self, url: &String) -> String {
    let re = Regex::new(r"\{\{ *([a-z\._]+) *\}\}").unwrap();

    let result = re.replace(url.as_str(), |caps: &Captures| {
      // TODO : multiple ?
      let capture = &caps[1];

      if let Some(item) = self.resolve_context_interpolation(&capture) {
        return item.to_string();
      }

      if let Some(item) = self.resolve_responses_interpolation(&capture) {
        return item.to_string();
      }

      panic!("{} Unknown '{}' variable!", "WARNING!".yellow().bold(), &capture);
    });

    self.base_url.to_string() + &result
  }

  // TODO: Refactor this function to support multiple levels
  fn resolve_responses_interpolation(&self, capture: &str) -> Option<String> {
    let cap_path: Vec<&str> = capture.split(".").collect();

    let (cap_root, cap_tail) = cap_path.split_at(1);

    match self.responses.get(cap_root[0]) {
      Some(value) => {
        return Some(value[cap_tail[0]].to_string());
      },
      _ => {
        None
      }
    }
  }

  // TODO: Refactor this function to support multiple levels
  fn resolve_context_interpolation(&self, capture: &str) -> Option<String> {
    let cap_path: Vec<&str> = capture.split(".").collect();

    let (cap_root, cap_tail) = cap_path.split_at(1);

    match self.context.get(cap_root[0]) {
      Some(value) => {
        if let Some(vs) = value.as_str() {
          return Some(vs.to_string());
        }

        if let Some(vi) = value.as_i64() {
          return Some(vi.to_string());
        }

        if let Some(vh) = value.as_hash() {
          let item_key = Yaml::String(cap_tail[0].to_string());

          match vh.get(&item_key){
            Some(value) => {
              if let Some(vs) = value.as_str() {
                return Some(vs.to_string());
              }

              if let Some(vi) = value.as_i64() {
                return Some(vi.to_string());
              }

              panic!("{} Unknown type for '{}' variable!", "WARNING!".yellow().bold(), &capture);
            },
            _ => {
              panic!("{} Unknown '{}' variable!", "WARNING!".yellow().bold(), &capture);
            }
          }
        }

        panic!("{} Unknown type for '{}' variable!", "WARNING!".yellow().bold(), &capture);
      },
      _ => {
        None
      }
    }
  }
}
