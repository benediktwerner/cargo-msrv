extern crate cargo_msrv;

use parameterized::parameterized;
use rust_releases::{semver, Release};

use crate::common::fixtures_path;
use cargo_msrv::MinimalCompatibility;
use common::{
    run_cargo_version_which_doesnt_support_lockfile_v2, run_msrv, run_msrv_with_releases,
};

mod common;

#[parameterized(
    folder = {
        "1.35.0",
        "1.36.0",
        "1.37.0",
        "1.38.0",
    },
    expected_version = {
        semver::Version::new(1,35,0),
        semver::Version::new(1,36,0),
        semver::Version::new(1,37,0),
        semver::Version::new(1,38,0),
    }
)]
fn msrv_using_linear_method(folder: &str, expected_version: semver::Version) {
    let folder = fixtures_path().join(folder);

    let with_args = vec!["cargo-msrv", "--linear", "--path", folder.to_str().unwrap()];

    let result = run_msrv(with_args);
    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}

#[parameterized(
    folder = {
        "1.35.0",
        "1.36.0",
        "1.37.0",
        "1.38.0",
    },
    expected_version = {
        semver::Version::new(1,35,0),
        semver::Version::new(1,36,0),
        semver::Version::new(1,37,0),
        semver::Version::new(1,38,0),
    }
)]
fn msrv_using_bisect_method(folder: &str, expected_version: semver::Version) {
    let folder = fixtures_path().join(folder);

    let with_args = vec!["cargo-msrv", "--bisect", "--path", folder.to_str().unwrap()];

    let result = run_msrv(with_args);
    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}

#[test]
fn msrv_unsupported() {
    let folder = fixtures_path().join("unbuildable");

    let with_args = vec!["cargo-msrv", "--path", folder.to_str().unwrap()];

    let result = run_msrv(with_args);
    assert_eq!(result, MinimalCompatibility::NoCompatibleToolchains);
}

#[parameterized(
    folder = {
        "1.35.0",
        "1.36.0",
        "1.37.0",
        "1.38.0",
    },
    expected_version = {
        semver::Version::new(1,35,0),
        semver::Version::new(1,36,0),
        semver::Version::new(1,37,0),
        semver::Version::new(1,38,0),
    }
)]
fn msrv_with_custom_command(folder: &str, expected_version: semver::Version) {
    let folder = fixtures_path().join(folder);

    let with_args = vec![
        "cargo-msrv",
        "--linear",
        "--path",
        folder.to_str().unwrap(),
        "--",
        "cargo",
        "check",
    ];

    let result = run_msrv(with_args);
    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}

#[parameterized(
    release_source = {
        "rust-changelog",
        "rust-dist"
    },
    folder = {
        "1.38.0",
        "1.38.0"
    },
    expected_version = {
        semver::Version::new(1,38,0),
        semver::Version::new(1,38,0),
    }
)]
fn msrv_with_release_source(release_source: &str, folder: &str, expected_version: semver::Version) {
    let folder = fixtures_path().join(folder);

    let with_args = vec![
        "cargo-msrv",
        "--linear",
        "--release-source",
        release_source,
        "--path",
        folder.to_str().unwrap(),
        "--",
        "cargo",
        "check",
    ];

    let result = run_msrv(with_args);

    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}

#[test]
fn msrv_with_old_lockfile() {
    let folder = fixtures_path().join("1.29.2");
    let with_args = vec![
        "cargo-msrv",
        "--linear",
        "--path",
        folder.to_str().unwrap(),
        "--ignore-lockfile",
    ];

    let result = run_cargo_version_which_doesnt_support_lockfile_v2(with_args);
    assert_eq!(result.to_version().minor, 29);
}

mod minimum_from_edition {
    use super::{run_msrv_with_releases, semver, Release};
    use crate::fixtures_path;

    #[test]
    fn msrv_min_with_edition_in_cargo_toml() {
        let folder = fixtures_path().join("1.30.0");

        let with_args = vec!["cargo-msrv", "--linear", "--path", folder.to_str().unwrap()];

        let versions = vec![
            Release::new_stable(semver::Version::new(1, 32, 0)),
            Release::new_stable(semver::Version::new(1, 31, 0)),
            Release::new_stable(semver::Version::new(1, 30, 0)),
            Release::new_stable(semver::Version::new(1, 29, 0)),
        ];
        let (result, reporter) = run_msrv_with_releases(with_args, versions);
        assert_eq!(result.to_version().minor, 31);
        assert_eq!(
            reporter.expose_successes(),
            vec![
                (true, semver::Version::new(1, 32, 0)),
                (true, semver::Version::new(1, 31, 0)),
            ]
        );
    }

    #[test]
    fn msrv_no_minimum_with_flag() {
        let folder = fixtures_path().join("1.30.0");

        let with_args = vec![
            "cargo-msrv",
            "--linear",
            "--path",
            folder.to_str().unwrap(),
            "--no-read-min-edition",
        ];

        let versions = vec![
            Release::new_stable(semver::Version::new(1, 32, 0)),
            Release::new_stable(semver::Version::new(1, 31, 0)),
            Release::new_stable(semver::Version::new(1, 30, 0)),
            Release::new_stable(semver::Version::new(1, 29, 0)),
        ];
        let (result, reporter) = run_msrv_with_releases(with_args, versions);
        assert_eq!(result.to_version().minor, 31);
        assert_eq!(
            reporter.expose_successes(),
            vec![
                (true, semver::Version::new(1, 32, 0)),
                (true, semver::Version::new(1, 31, 0)),
                (false, semver::Version::new(1, 30, 0)),
            ]
        );
    }
}

#[parameterized(
    package = {
        "a",
        "b",
    },
    expected_version = {
        semver::Version::new(1,56,1), // `a` has an MSRV of 1.56.1
        semver::Version::new(1,58,1), // `b` has an MSRV of 1.58.1
    }
)]
fn msrv_in_a_virtual_workspace_default_check_command(
    package: &str,
    expected_version: semver::Version,
) {
    let folder = fixtures_path().join("virtual-workspace").join(package);
    let folder = folder.to_str().unwrap();

    let with_args = vec!["cargo-msrv", "--path", folder];

    let versions = vec![
        Release::new_stable(semver::Version::new(1, 58, 1)),
        Release::new_stable(semver::Version::new(1, 56, 1)),
    ];
    let (result, _) = run_msrv_with_releases(with_args, versions);
    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}

#[parameterized(
    command = {
        "cargo check",
        "cargo check --workspace",
        "cargo check",
        "cargo check --workspace",
    },
    package = {
        "a",
        "a",
        "b",
        "b",
    },
    expected_version = {
        semver::Version::new(1,56,1), // `a` has an MSRV of 1.56.1
        semver::Version::new(1,58,1), // since `b` has a greater MSRV than `a`, the greatest common MSRV of the workspace is the MSRV of `b`: 1.58.1
        semver::Version::new(1,58,1), // `b` has an MSRV of 1.58.1
        semver::Version::new(1,58,1), // the greatest common MSRV of the workspace is the MSRV of `b`: 1.58.1
    }
)]
fn msrv_in_a_virtual_workspace(command: &str, package: &str, expected_version: semver::Version) {
    let folder = fixtures_path().join("virtual-workspace").join(package);
    let folder = folder.to_str().unwrap();

    let base_command = vec!["cargo-msrv", "--path", folder, "--"];
    let custom_check_command = command.split_ascii_whitespace().collect::<Vec<_>>();
    let command = vec![base_command, custom_check_command];

    let with_args = command.iter().flatten().collect::<Vec<_>>();

    let versions = vec![
        Release::new_stable(semver::Version::new(1, 58, 1)),
        Release::new_stable(semver::Version::new(1, 56, 1)),
    ];
    let (result, _) = run_msrv_with_releases(with_args, versions);
    let actual_version = result.to_version();

    assert_eq!(actual_version, expected_version);
}
