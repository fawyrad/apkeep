use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

arg_enum! {
    pub enum DownloadSource {
        APKPure,
        GooglePlay,
        FDroid,
    }
}

pub fn app() -> App<'static, 'static> {
    App::new(format!("apkeep v{}", VERSION))
        .author("William Budington <bill@eff.org>")
        .about("Downloads APKs from various sources")
        .usage("apkeep <-a app_id[@version] | -c csv [-f field] [-v version_field]> [-d download_source] [-r parallel] OUTPATH")
        .arg(
            Arg::with_name("app")
                .help("Provide the ID and optionally the version of an app directly (e.g. com.instagram.android)")
                .short("a")
                .long("app")
                .takes_value(true)
                .conflicts_with("csv")
                .required_unless("csv"),
        )
        .arg(
            Arg::with_name("csv")
                .help("CSV file to use")
                .short("c")
                .long("csv")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("field")
                .help("CSV field containing app IDs (used only if CSV is specified)")
                .short("f")
                .long("field")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("version_field")
                .help("CSV field containing versions (used only if CSV is specified)")
                .short("v")
                .long("version-field")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("list_versions")
                .help("List the versions available")
                .short("l")
                .long("list-versions")
                .required(false),
        )
        .arg(
            Arg::with_name("download_source")
                .help("Where to download the APKs from")
                .short("d")
                .long("download-source")
                .default_value("APKPure")
                .takes_value(true)
                .possible_values(&DownloadSource::variants())
                .required(false),
        )
        .arg(
            Arg::with_name("google_username")
                .help("Google Username (required if download source is Google Play)")
                .short("u")
                .long("username")
                .takes_value(true)
                .required_if("download_source", "GooglePlay"),
        )
        .arg(
            Arg::with_name("google_password")
                .help("Google App Password (required if download source is Google Play)")
                .short("p")
                .long("password")
                .takes_value(true)
                .required_if("download_source", "GooglePlay"),
        )
        .arg(
            Arg::with_name("sleep_duration")
                .help("Sleep duration (in ms) before download requests")
                .short("s")
                .long("sleep-duration")
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("parallel")
                .help("The number of parallel APK fetches to run at a time")
                .short("r")
                .long("parallel")
                .takes_value(true)
                .default_value("4")
                .required(false),
        )
        .arg(
            Arg::with_name("OUTPATH")
                .help("Path to store output files")
                .index(1),
        )
}
