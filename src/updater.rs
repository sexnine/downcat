use crate::info;
use colored::Colorize;
use self_update::{
    update::{Release, ReleaseUpdate},
    Status,
};

fn get_update() -> Result<Box<dyn ReleaseUpdate>, self_update::errors::Error> {
    self_update::backends::github::Update::configure()
        .repo_owner("sexnine")
        .repo_name("downcat")
        .bin_name("downcat")
        .show_download_progress(true)
        .no_confirm(true)
        .show_output(false)
        .current_version(info::version())
        .build()
}

fn get_latest_release() -> Result<Release, self_update::errors::Error> {
    match get_update() {
        Ok(x) => x.get_latest_release(),
        Err(e) => Err(e),
    }
}

fn is_newer_version() -> Option<String> {
    match get_latest_release() {
        Ok(x) => {
            let latest_version = x.version;
            if self_update::version::bump_is_greater(info::version(), latest_version.as_str())
                .unwrap_or(false)
            {
                Some(latest_version)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn check_for_update() {
    // async {
    if let Some(latest_version) = is_newer_version() {
        println!(
            "\n{} {}\n",
            "🎉 A new version is available!".bright_green(),
            format!("{} -> {}", info::version(), latest_version).bright_magenta(),
        );
        #[cfg(not(feature = "no-self-update"))]
        println!(
            "{}{}\n",
            "   You can update by running ".bright_green(),
            "downcat update".bright_blue()
        )
    }
    // }
}

pub fn update() {
    println!("{}", "\n🐈 Starting update...\n".dimmed());

    let update = match get_update() {
        Ok(x) => x,
        _ => {
            println!(
                "{}",
                "❌ An unexpected error occoured while updating downcat".bright_red()
            );
            return;
        }
    };

    let status = update.update();
    println!("");

    match status {
        Ok(s) => match s {
            Status::Updated(v) => println!(
                "{}",
                format!("✅ Sucessfully updated downcat to v{}!", v).bright_green()
            ),
            Status::UpToDate(v) => println!(
                "{}",
                format!("✅ Downcat is already on the latest version: v{}", v).bright_green()
            ),
        },
        Err(e) => {
            println!(
                "{}",
                format!("❌ Error while updating downcat: {e}").bright_red()
            )
        }
    }
}
