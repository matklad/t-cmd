use std::{process::Command, time::Instant};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(-1)
    }
}

fn try_main() -> Result<(), String> {
    let mut args = std::env::args_os();
    args.next();
    let progn = args.next().ok_or_else(|| "missing program name".to_string())?;
    let mut cmd = Command::new(progn);
    cmd.args(args);
    let t = Instant::now();
    let status = cmd.status().map_err(|err| format!("failed to run command: {}", err))?;
    let real_time = t.elapsed();

    let ru = rusage::children().map_err(|()| "failed to get rusage".to_string())?;

    eprintln!(
        "
real {:.2?}
cpu  {:.2?} ({:.2?} user + {:.2?} sys)
rss  {:.2}mb",
        real_time,
        (ru.user_cpu + ru.sys_cpu),
        ru.user_cpu,
        ru.sys_cpu,
        (ru.max_rss_bytes as f64 / (1024.0 * 1024.0))
    );
    if !status.success() {
        return Err(match status.code() {
            Some(code) => format!("\ncommand exited with non-zero code: {}", code),
            None => "\ncommand was terminated by signal".to_string(),
        });
    }
    Ok(())
}

mod rusage {
    #![allow(non_camel_case_types)]

    use std::time::Duration;

    #[repr(C)]
    #[derive(Default)]
    struct timeval {
        tv_sec: i64,
        tv_usec: i64,
    }

    impl From<timeval> for Duration {
        fn from(tv: timeval) -> Self {
            Duration::new(tv.tv_sec as u64, tv.tv_usec as u32 * 1_000)
        }
    }

    #[repr(C)]
    #[derive(Default)]
    struct rusage {
        ru_utime: timeval,
        ru_stime: timeval,
        ru_maxrss: i64,
        ru_ixrss: i64,
        ru_idrss: i64,
        ru_isrss: i64,
        ru_minflt: i64,
        ru_majflt: i64,
        ru_nswap: i64,
        ru_inblock: i64,
        ru_oublock: i64,
        ru_msgsnd: i64,
        ru_msgrcv: i64,
        ru_nsignals: i64,
        ru_nvcsw: i64,
        ru_nivcsw: i64,
    }
    const RUSAGE_CHILDREN: i32 = -1;
    extern "C" {
        fn getrusage(who: i32, rusage: &mut rusage) -> i32;
    }

    pub(super) struct ResourceUsage {
        pub(super) sys_cpu: Duration,
        pub(super) user_cpu: Duration,
        pub(super) max_rss_bytes: u64,
    }

    pub(super) fn children() -> Result<ResourceUsage, ()> {
        let mut ru = rusage::default();
        let ret = unsafe { getrusage(RUSAGE_CHILDREN, &mut ru) };
        if ret != 0 {
            return Err(());
        }
        Ok(ResourceUsage {
            sys_cpu: ru.ru_stime.into(),
            user_cpu: ru.ru_utime.into(),
            max_rss_bytes: ru.ru_maxrss as u64 * if cfg!(target_os = "macos") { 1 } else { 1024 },
        })
    }
}
