// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Sanchayan Maity

use zbus::blocking::Connection;
use zbus::zvariant::Value;
use zbus::Result;

const RTKIT_OBJECT_PATH: &str = "/org/freedesktop/RealtimeKit1";
const RTKIT_SERVICE_NAME: &str = "org.freedesktop.RealtimeKit1";
const RTKIT_INTERFACE: &str = "org.freedesktop.RealtimeKit1";

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

    pub fn make_thread_high_priority(&self, thread_id: u64, priority: i32) -> anyhow::Result<()> {
        self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some(RTKIT_INTERFACE),
            "MakeThreadHighPriority",
            &(thread_id, priority),
        )?;

        Ok(())
    }

    pub fn make_thread_high_priority_with_pid(
        &self,
        process_id: u64,
        thread_id: u64,
        priority: i32,
    ) -> anyhow::Result<()> {
        self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some(RTKIT_INTERFACE),
            "MakeThreadHighPriorityWithPID",
            &(process_id, thread_id, priority),
        )?;

        Ok(())
    }

    pub fn make_thread_realtime(&self, thread_id: u64, priority: u32) -> anyhow::Result<()> {
        self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some(RTKIT_INTERFACE),
            "MakeThreadRealtime",
            &(thread_id, priority),
        )?;

        Ok(())
    }

    pub fn make_thread_realtime_with_pid(
        &self,
        process_id: u64,
        thread_id: u64,
        priority: u32,
    ) -> anyhow::Result<()> {
        self.connection.call_method(
            Some(RTKIT_SERVICE_NAME),
            RTKIT_OBJECT_PATH,
            Some(RTKIT_INTERFACE),
            "MakeThreadRealtimeWithPID",
            &(process_id, thread_id, priority),
        )?;

        Ok(())
    }

    pub fn current_thread_id() -> u64 {
        unsafe { libc::syscall(libc::SYS_gettid) as u64 }
    }

    pub fn current_process_id() -> u64 {
        std::process::id() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sched_attr() -> anyhow::Result<libc::sched_attr> {
        unsafe {
            let mut attr: libc::sched_attr = std::mem::MaybeUninit::zeroed().assume_init();

            let ret = libc::syscall(
                libc::SYS_sched_getattr,
                0,
                &mut attr as *mut libc::sched_attr,
                std::mem::size_of::<libc::sched_attr>(),
                0,
            );

            if ret < 0 {
                Err(std::io::Error::last_os_error().into())
            } else {
                Ok(attr)
            }
        }
    }

    #[test]
    fn test_property() {
        let rtkit = RTKit::new().unwrap();

        // Test for default values
        assert_eq!(rtkit.max_realtime_priority().unwrap(), 20);
        assert_eq!(rtkit.min_nice_level().unwrap(), -15);
        assert_eq!(rtkit.rttime_usec_max().unwrap(), 200000);
    }

    #[test]
    fn test_thread_id_retrieval() {
        assert!(RTKit::current_thread_id() > 0);
    }

    #[test]
    fn test_process_id_retrieval() {
        assert!(RTKit::current_process_id() > 0);
    }

    #[test]
    fn test_make_thread_high_priority() {
        let rtkit = RTKit::new().unwrap();

        let thread_id = RTKit::current_thread_id();
        assert!(rtkit.make_thread_high_priority(thread_id, -10).is_ok());

        let attr = get_sched_attr().unwrap();
        assert_eq!(attr.sched_nice, -10);
    }

    #[test]
    fn test_make_thread_high_priority_with_pid() {
        let rtkit = RTKit::new().unwrap();

        let pid = RTKit::current_process_id();
        let thread_id = RTKit::current_thread_id();
        rtkit
            .make_thread_high_priority_with_pid(pid, thread_id, -10)
            .unwrap();

        let attr = get_sched_attr().unwrap();
        assert_eq!(attr.sched_nice, -10);
    }
}
