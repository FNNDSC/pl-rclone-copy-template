/// WARNING: this code sucks because it expects to be used in the context of _ChRIS_
/// and not directly by a terminal user.
///
/// All error handling is done by `panic!`
use phf::phf_map;
use std::env;
use std::process::{Command, ExitCode};

const IGNORED_ARGS: [&str; 2] = ["--saveinputmeta", "--saveoutputmeta"];

/// Parameters of `rclone` accepted by this program and given to `rclone` transparently,
/// and the number of arguments they consume.
static PARAM_MAP: phf::Map<&'static str, usize> = phf_map! {
    "--ignore-case" => 1,
    "--ignore-checksum" => 1,
    "--ignore-errors" => 1,
    "--ignore-existing" => 1,
    "--timeout" => 2,
    "--include" => 2,
    "--exclude" => 2,
    "--filter" => 2,
    "--include-from" => 2,
    "--exclude-from" => 2,
    "--filter-from" => 2,
    "--max-depth" => 2,
    "--max-size" => 2,
    "--max-name-length" => 2,
    "--min-age" => 2,
    "--min-size" => 2,
    "--error-on-no-transfer" => 1,
    "--fast-list" => 1,
};

const PATH_FLAG: &str = "--path";
const NO_IMMUTABLE_FLAG: &str = "--no-immutable";

#[derive(Debug, PartialEq)]
enum UsedAs {
    Invalid,
    Fs(usize),
    Ds(usize, usize),
}

enum RcloneWrapper {
    Real,
    Mock,
}

impl RcloneWrapper {
    fn run(&self, args: &[String]) -> i32 {
        eprintln!("$> rclone {}", args.join(" "));

        match self {
            RcloneWrapper::Real => Command::new("rclone")
                .args(args)
                .status()
                .unwrap_or_else(|_| panic!("failed to run `{:?}`", args))
                .code()
                .expect("rclone was terminated by signal."),
            RcloneWrapper::Mock => 0,
        }
    }

    fn use_default_remote(&self, path: String) -> String {
        if path.contains(':') {
            return path;
        }
        format!("{}{}", self.get_first_remote(), path)
    }

    fn get_first_remote(&self) -> String {
        let remotes = self.listremotes();
        let (first_remote, _) = remotes
            .split_once('\n')
            .expect("Cannot parse output of `rclone listremotes`");
        first_remote.to_string()
    }

    fn listremotes(&self) -> String {
        match self {
            RcloneWrapper::Real => {
                let bytes = Command::new("rclone")
                    .args(["listremotes"])
                    .output()
                    .expect("failed to run `rclone listremotes`")
                    .stdout;
                String::from_utf8_lossy(&*bytes).to_string()
            }
            RcloneWrapper::Mock => "mock_remote_name:\n".into(),
        }
    }
}

fn main() -> ExitCode {
    let wrapper = RcloneWrapper::Real;
    let mut args = env::args()
        .filter(|s| !IGNORED_ARGS.contains(&s.as_str()))
        .collect::<Vec<String>>();
    let remote_path = wrapper.use_default_remote(remove_remote_path(&mut args));

    let mode = get_positionals(&args);
    replace_at(&mut args, &mode, remote_path);
    let mut args = flip_immutable_flag(args);

    args.push("--verbose".into());
    // can we use --metadata to preserve group read-write permissions?
    // https://rclone.org/docs/#metadata

    args[0] = "copy".into();
    let rc = wrapper.run(&args[..]);
    ExitCode::from(rc as u8)
}

fn remove_remote_path(args: &mut Vec<String>) -> String {
    for (i, s) in args.iter().enumerate() {
        if s.as_str() == PATH_FLAG {
            let value = args.remove(i + 1);
            args.remove(i);
            return value;
        }
    }
    panic!("missing {} option", PATH_FLAG)
}

fn get_positionals(args: &[String]) -> UsedAs {
    let i = 1;
    if let Some(incoming) = get_next_positional_index(args, i) {
        if let Some(outgoing) = get_next_positional_index(args, incoming + 1) {
            return UsedAs::Ds(incoming, outgoing);
        }
        return UsedAs::Fs(incoming);
    }
    UsedAs::Invalid
}

fn get_next_positional_index(args: &[String], i: usize) -> Option<usize> {
    if i >= args.len() {
        return None;
    }
    if args[i] == NO_IMMUTABLE_FLAG {
        // special case
        return get_next_positional_index(args, i + 1);
    }
    if let Some(inc) = PARAM_MAP.get(&*args[i]) {
        return get_next_positional_index(args, i + inc);
    }
    Some(i)
}

fn replace_at(args: &mut Vec<String>, mode: &UsedAs, remote_path: String) {
    match mode {
        UsedAs::Fs(outgoing) => {
            let last_index = args.len();
            args.push(remote_path);
            args.swap(*outgoing, last_index);
        }
        UsedAs::Ds(_, outgoing) => {
            args[*outgoing] = remote_path;
        }
        UsedAs::Invalid => {
            panic!("No positional arguments found")
        }
    }
}

/// Remove the `--no-immutable` item if present, otherwise add `--immutable`.
///
/// `rclone` has a dangerous default behavior which allows files to be overwritten,
/// which can be disabled by using the `--immutable` option. This app flips the
/// default behavior, passing `--immutable` to `rclone` unless `--no-immutable`
/// is specified.
fn flip_immutable_flag(mut args: Vec<String>) -> Vec<String> {
    if let Some(i) = args.iter().enumerate().find_map(|(i, a)| {
        if a == NO_IMMUTABLE_FLAG {
            Some(i)
        } else {
            None
        }
    }) {
        args.remove(i);
    } else {
        args.push("--immutable".into());
    }
    args
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    const MOCK_WRAPPER: RcloneWrapper = RcloneWrapper::Mock;

    #[rstest]
    #[case("/neuro/my_data", "mock_remote_name:/neuro/my_data")]
    #[case("storage:/neuro/my_data", "storage:/neuro/my_data")]
    fn test_use_default_remote(#[case] path: &str, #[case] expected: &str) {
        let actual = MOCK_WRAPPER.use_default_remote(path.into());
        assert_eq!(actual, expected.to_string())
    }

    #[rstest]
    #[case(
    vec!["chrclone", "--path", "/neuro/my_data", "/share/incoming", "/share/outgoing"],
    vec!["chrclone", "/share/incoming", "/share/outgoing"],
    "/neuro/my_data"
    )]
    fn test_remove_remote_path(
        #[case] cmd: Vec<&str>,
        #[case] after: Vec<&str>,
        #[case] expected: &str,
    ) {
        let mut args = list_string(cmd);
        let actual = remove_remote_path(&mut args);
        assert_eq!(actual, expected);
        assert_eq!(args, after);
    }

    #[rstest]
    #[case(
    vec!["chrclone", "--path", "/neuro/my_data", "/share/incoming", "/share/outgoing"],
    vec!["chrclone", "--path", "/neuro/my_data", "/share/incoming", "/share/outgoing", "--immutable"],
    )]
    #[case(
    vec!["chrclone", "--path", "/neuro/my_data", "--no-immutable", "/share/incoming", "/share/outgoing"],
    vec!["chrclone", "--path", "/neuro/my_data", "/share/incoming", "/share/outgoing"],
    )]
    fn test_flip_immutable_flag(#[case] cmd: Vec<&str>, #[case] expected: Vec<&str>) {
        let actual = flip_immutable_flag(list_string(cmd));
        assert_eq!(actual, list_string(expected));
    }

    #[rstest]
    #[case(
    vec!["chrclone", "--ignore-case", "/share/incoming", "/share/outgoing"],
    UsedAs::Ds(2, 3)
    )]
    #[case(
    vec!["chrclone", "/share/incoming", "/share/outgoing", "--ignore-case"],
    UsedAs::Ds(1, 2)
    )]
    #[case(
    vec!["chrclone", "--ignore-case", "--no-immutable", "/share/incoming", "/share/outgoing"],
    UsedAs::Ds(3, 4),
    )]
    #[case(
    vec!["chrclone", "--ignore-case", "/share/outgoing"],
    UsedAs::Fs(2),
    )]
    #[case(
    vec!["chrclone", "--ignore-case"],
    UsedAs::Invalid
    )]
    fn test_get_positionals(#[case] data: Vec<&str>, #[case] mode: UsedAs) {
        assert_eq!(get_positionals(&list_string(data)), mode)
    }

    #[rstest]
    #[case(
    vec!["chrclone", "--ignore-case", "/share/incoming", "/share/outgoing"],
    UsedAs::Ds(2, 3),
    vec!["chrclone", "--ignore-case", "/share/incoming", "replaced"]
    )]
    #[case(
    vec!["chrclone", "/share/incoming", "/share/outgoing", "--ignore-case"],
    UsedAs::Ds(1, 2),
    vec!["chrclone", "/share/incoming", "replaced", "--ignore-case"]
    )]
    #[case(
    vec!["chrclone", "--ignore-case", "/share/outgoing"],
    UsedAs::Fs(2),
    vec!["chrclone", "--ignore-case", "replaced", "/share/outgoing"]
    )]
    fn test_replace_at(#[case] data: Vec<&str>, #[case] mode: UsedAs, #[case] expected: Vec<&str>) {
        let mut actual = list_string(data);
        replace_at(&mut actual, &mode, "replaced".into());
        assert_eq!(actual, list_string(expected))
    }

    fn list_string(x: Vec<&str>) -> Vec<String> {
        x.iter().map(|s| s.to_string()).collect()
    }
}
