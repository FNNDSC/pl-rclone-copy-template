use phf::phf_map;
use std::env;

const IGNORED_ARGS: [&str; 2] = ["--saveinputmeta", "--saveoutputmeta"];

static PARAM_MAP: phf::Map<&'static str, usize> = phf_map! {
    "--ignore-case" => 1,
    "--ignore-checksum" => 1,
    "--ignore-existing" => 1,
    "--timeout" => 1,
    "--include" => 2,
    "--exclude" => 2,
    "--filter" => 2,
    "--include-from" => 2,
    "--exclude-from" => 2,
    "--filter-from" => 2
};

#[derive(Debug, PartialEq)]
enum UsedAs {
    Invalid,
    Fs(usize),
    Ds(usize, usize),
}

fn main() {
    let mut args = env::args()
        .filter(|s| !IGNORED_ARGS.contains(&s.as_str()))
        .collect::<Vec<String>>();

    let mode = get_positionals(&args);
    let remote_path = "todo:/todo/todo/todo".to_string();
    replace_at(&mut args, &mode, remote_path);
    dbg!(args);
    // --immutable --verbose
    // can we use --metadata to preserve group read-write permissions?
    // https://rclone.org/docs/#metadata
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

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

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
    vec!["chrclone", "--ignore-case", "/share/outgoing"],
    UsedAs::Fs(2),
    )]
    #[case(
    vec!["chrclone", "--ignore-case"],
    UsedAs::Invalid
    )]
    fn test_get_positionals(#[case] data: Vec<&str>, #[case] mode: UsedAs) {
        assert_eq!(
            get_positionals(&list_string(data)),
            mode
        )
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
