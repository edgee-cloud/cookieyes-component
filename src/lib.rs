use std::collections::HashMap;

use exports::edgee::components::consent_management::{Consent, Dict, Guest};

wit_bindgen::generate!({
    path: ".edgee/wit",
    world: "consent-management",
    generate_all,
});

struct Component;
export!(Component);

impl Guest for Component {
    fn map(cookies: Dict, _settings: Dict) -> Option<Consent> {
        let cookies = match Cookies::try_from(cookies) {
            Ok(cookies) => cookies,
            Err(err) => {
                eprintln!("Could not get cookies: {err}");
                return Some(Consent::Pending);
            }
        };

        let Some(ref action) = cookies.action else {
            return Some(Consent::Pending);
        };
        if action != "yes" {
            return Some(Consent::Pending);
        }

        for value in cookies.extra_fields.values() {
            if value == "no" {
                return Some(Consent::Denied);
            }
        }

        Some(Consent::Granted)
    }
}

struct Cookies {
    action: Option<String>,
    extra_fields: HashMap<String, String>,
}

impl TryFrom<Dict> for Cookies {
    type Error = String;

    fn try_from(value: Dict) -> Result<Self, Self::Error> {
        let dict: HashMap<_, _> = value.into_iter().collect();

        let value = dict
            .get("cookieyes-consent")
            .ok_or_else(|| "Cookie not found: cookieyes-consent".to_string())?;
        let mut values = value
            .split(',')
            // .filter_map(|item| item.split_once(':'))
            .filter_map(|item| {
                let (key, value) = item.split_once(':')?;
                Some((key.to_string(), value.to_string()))
            })
            .collect::<HashMap<_, _>>();

        let action = values.remove("action");

        Ok(Self {
            action,
            extra_fields: values,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    macro_rules! dict {
        {
            $($key:literal: $value:expr),*$(,)?
        } => {
            vec![
                $(($key.to_string(), $value.to_string()),)*
            ]
        };
    }
    macro_rules! map {
        {
            $($key:literal: $value:expr),*$(,)?
        } => {
            {
                #[allow(unused_mut)]
                let mut m = std::collections::HashMap::new();
                $(m.insert($key.to_string(), $value.to_string());)*
                m
            }
        };
    }

    fn make_cookie(values: HashMap<String, String>) -> String {
        values
            .into_iter()
            .map(|(key, value)| format!("{key}:{value}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[test]
    fn test_consent_pending() {
        let cookies = dict! {};

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Pending)
        );
    }

    #[test]
    fn test_consent_pending_with_action() {
        let cookies = dict! {
            "cookieyes-consent": make_cookie(map! {
                "action": "pending",
            }),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Pending)
        );
    }

    #[test]
    fn test_consent_granted() {
        let cookies = dict! {
            "cookieyes-consent": make_cookie(map! {
                "action": "yes",
            }),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Granted)
        );
    }

    #[test]
    fn test_consent_denied() {
        let cookies = dict! {
            "cookieyes-consent": make_cookie(map! {
                "action": "yes",
                "granted": "no",
            }),
        };

        assert_eq!(
            Component::map(cookies, Default::default()),
            Some(Consent::Denied)
        );
    }
}
