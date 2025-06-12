// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Sanchayan Maity

use zbus::Result;
use zbus::blocking::Connection;
use zbus::zvariant::Value;

const RTKIT_OBJECT_PATH: &str = "/org/freedesktop/RealtimeKit1";
const RTKIT_SERVICE_NAME: &str = "org.freedesktop.RealtimeKit1";

fn is_rtkit_available(connection: &Connection) -> Result<bool> {
    let message = connection.call_method(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        Some("org.freedesktop.DBus"),
        "ListNames",
        &(),
    )?;

    let names: Vec<String> = message.body().deserialize()?;

    Ok(names.contains(&"org.freedesktop.RealtimeKit1".to_string()))
}

pub struct RTKit {
    connection: Connection,
}

impl RTKit {
    pub fn new() -> anyhow::Result<RTKit> {
        let connection = Connection::system()?;

        is_rtkit_available(&connection)?;

        Ok(RTKit { connection })
    }

    pub fn max_realtime_priority(&self) -> anyhow::Result<i32> {
        match self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.freedesktop.RealtimeKit1", "MaxRealtimePriority"),
        ) {
            Ok(message) => {
                let body = message.body().clone().to_owned();
                let variant: Result<Value> = body.deserialize();
                match variant {
                    Ok(value) => Ok(i32::try_from(&value).unwrap()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn min_nice_level(&self) -> anyhow::Result<i32> {
        match self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.freedesktop.RealtimeKit1", "MinNiceLevel"),
        ) {
            Ok(message) => {
                let body = message.body().clone().to_owned();
                let variant: Result<Value> = body.deserialize();
                match variant {
                    Ok(value) => Ok(i32::try_from(&value).unwrap()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn rttime_usec_max(&self) -> anyhow::Result<i64> {
        match self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &("org.freedesktop.RealtimeKit1", "RTTimeUSecMax"),
        ) {
            Ok(message) => {
                let body = message.body().clone().to_owned();
                let variant: Result<Value> = body.deserialize();
                match variant {
                    Ok(value) => Ok(i64::try_from(&value).unwrap()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        let rtkit = RTKit::new();
        assert!(rtkit.is_ok());
        let rtkit = rtkit.unwrap();

        // Test for default values
        assert_eq!(rtkit.max_realtime_priority().unwrap(), 20);
        assert_eq!(rtkit.min_nice_level().unwrap(), -15);
        assert_eq!(rtkit.rttime_usec_max().unwrap(), 200000);
    }
}
