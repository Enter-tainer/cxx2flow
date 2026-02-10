use std::{
    fs,
    path::{Path, PathBuf},
};

use cxx2flow_lib::{
    display::{GraphDisplayBackend, d2::D2, dot::Dot, tikz::Tikz},
    generate,
};
use libtest_mimic::{Arguments, Failed, Trial};

#[derive(Clone, Debug)]
struct FixtureCase {
    name: String,
    function: String,
    path: PathBuf,
    source: Vec<u8>,
}

#[derive(Clone, Copy)]
enum BackendKind {
    DotPolyline,
    DotCurly,
    D2,
    Tikz,
}

#[derive(Clone, Copy)]
struct ErrorCase {
    snapshot_name: &'static str,
    fixture_name: &'static str,
    function: &'static str,
}

const DOT_CURLY_CASES: &[&str] = &[
    "if_else",
    "else_if_chain",
    "switch_with_default",
    "goto_forward_label",
    "stacked_labels",
    "switch_char_literal",
    "switch_negative_case",
    "switch_default_middle",
    "enum_class_switch",
    "namespace_function",
    "nested_namespace_function",
];

const TIKZ_CASES: &[&str] = &[
    "linear_return",
    "while_continue_break",
    "nested_loop_mix",
    "range_for",
    "range_for_const_ref",
    "multi_function_pick_second",
    "class_method_and_main",
    "template_function",
    "range_for_initializer",
];

const ERROR_CASES: &[ErrorCase] = &[
    ErrorCase {
        snapshot_name: "error__missing_function_debug",
        fixture_name: "linear_return",
        function: "missing_function",
    },
    ErrorCase {
        snapshot_name: "error__unexpected_continue_debug",
        fixture_name: "unexpected_continue",
        function: "main",
    },
    ErrorCase {
        snapshot_name: "error__unexpected_break_debug",
        fixture_name: "unexpected_break",
        function: "main",
    },
];

const KNOWN_BROKEN_CASES: &[&str] = &[];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn snapshot_fixtures_dir() -> PathBuf {
    project_root()
        .join("tests")
        .join("fixtures")
        .join("snapshots")
}

fn error_fixtures_dir() -> PathBuf {
    project_root().join("tests").join("fixtures").join("errors")
}

fn split_fixture_stem(stem: &str) -> (String, String) {
    if let Some((name, function)) = stem.split_once("__") {
        (name.to_owned(), function.to_owned())
    } else {
        (stem.to_owned(), "main".to_owned())
    }
}

fn is_c_or_cpp(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };
    ext == "c" || ext == "cpp"
}

fn collect_snapshot_cases() -> Vec<FixtureCase> {
    let mut paths = fs::read_dir(snapshot_fixtures_dir())
        .unwrap_or_else(|error| panic!("failed to read snapshot fixtures: {error}"))
        .map(|entry| {
            entry
                .unwrap_or_else(|error| panic!("failed to read fixture entry: {error}"))
                .path()
        })
        .filter(|path| is_c_or_cpp(path))
        .collect::<Vec<_>>();

    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let cases = paths
        .into_iter()
        .map(|path| {
            let stem = path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_else(|| panic!("invalid fixture filename: {}", path.display()));
            let (name, function) = split_fixture_stem(stem);
            let source = fs::read(&path).unwrap_or_else(|error| {
                panic!("failed to read fixture {}: {error}", path.display())
            });

            FixtureCase {
                name,
                function,
                path,
                source,
            }
        })
        .collect::<Vec<_>>();

    assert!(!cases.is_empty(), "no snapshot fixtures found");
    cases
}

fn read_error_fixture(name: &str) -> (Vec<u8>, String) {
    let candidates = [
        error_fixtures_dir().join(format!("{name}.c")),
        error_fixtures_dir().join(format!("{name}.cpp")),
        snapshot_fixtures_dir().join(format!("{name}.c")),
        snapshot_fixtures_dir().join(format!("{name}.cpp")),
    ];
    for path in candidates {
        if path.exists() {
            let content = fs::read(&path).unwrap_or_else(|error| {
                panic!("failed to read error fixture {}: {error}", path.display())
            });
            let file_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_else(|| panic!("invalid error fixture filename: {}", path.display()))
                .to_owned();
            return (content, file_name);
        }
    }
    panic!("missing error fixture: {name}");
}

fn render(case: &FixtureCase, backend: GraphDisplayBackend) -> String {
    let file_name = case
        .path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_else(|| panic!("invalid fixture filename: {}", case.path.display()));
    generate(
        &case.source,
        file_name,
        Some(case.function.clone()),
        backend,
    )
    .unwrap_or_else(|error| panic!("failed to render case {}: {error:?}", case.name))
}

fn find_case<'a>(cases: &'a [FixtureCase], name: &str) -> &'a FixtureCase {
    cases
        .iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("unknown test case: {name}"))
}

fn should_skip_case(case_name: &str) -> bool {
    KNOWN_BROKEN_CASES.contains(&case_name)
}

fn run_snapshot_case(case: FixtureCase, backend: BackendKind) -> Result<(), Failed> {
    let (snapshot_name, output) = match backend {
        BackendKind::DotPolyline => (
            format!("dot_polyline__{}", case.name),
            render(&case, Dot::new(false).into()),
        ),
        BackendKind::DotCurly => (
            format!("dot_curly__{}", case.name),
            render(&case, Dot::new(true).into()),
        ),
        BackendKind::D2 => (
            format!("d2__{}", case.name),
            render(&case, D2::new().into()),
        ),
        BackendKind::Tikz => (
            format!("tikz__{}", case.name),
            render(&case, Tikz::new().into()),
        ),
    };
    insta::assert_snapshot!(snapshot_name, output);
    Ok(())
}

fn run_error_case(case: ErrorCase) -> Result<(), Failed> {
    let (content, file_name) = read_error_fixture(case.fixture_name);
    let error = generate(
        &content,
        &file_name,
        Some(case.function.to_owned()),
        Dot::new(false).into(),
    )
    .expect_err("error fixture should return an error");
    insta::assert_snapshot!(case.snapshot_name, format!("{error:?}"));
    Ok(())
}

fn build_trials() -> Vec<Trial> {
    let cases = collect_snapshot_cases();
    let mut trials = Vec::new();

    for case in &cases {
        if should_skip_case(&case.name) {
            continue;
        }
        let case = case.clone();
        let name = format!("dot_polyline::{}", case.name);
        trials.push(Trial::test(name, move || {
            run_snapshot_case(case, BackendKind::DotPolyline)
        }));
    }

    for case in &cases {
        if should_skip_case(&case.name) {
            continue;
        }
        let case = case.clone();
        let name = format!("d2::{}", case.name);
        trials.push(Trial::test(name, move || {
            run_snapshot_case(case, BackendKind::D2)
        }));
    }

    for case_name in DOT_CURLY_CASES {
        if should_skip_case(case_name) {
            continue;
        }
        let case = find_case(&cases, case_name).clone();
        let name = format!("dot_curly::{}", case.name);
        trials.push(Trial::test(name, move || {
            run_snapshot_case(case, BackendKind::DotCurly)
        }));
    }

    for case_name in TIKZ_CASES {
        if should_skip_case(case_name) {
            continue;
        }
        let case = find_case(&cases, case_name).clone();
        let name = format!("tikz::{}", case.name);
        trials.push(Trial::test(name, move || {
            run_snapshot_case(case, BackendKind::Tikz)
        }));
    }

    for case in ERROR_CASES {
        let case = *case;
        let name = format!("error::{}", case.snapshot_name);
        trials.push(Trial::test(name, move || run_error_case(case)));
    }

    trials
}

fn main() {
    let args = Arguments::from_args();
    let trials = build_trials();
    libtest_mimic::run(&args, trials).exit();
}
