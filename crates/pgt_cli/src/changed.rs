use crate::CliDiagnostic;
use pgt_configuration::PartialConfiguration;
use pgt_fs::FileSystem;
use pgt_workspace::DynRef;
use std::ffi::OsString;

pub(crate) fn get_changed_files(
    fs: &DynRef<'_, dyn FileSystem>,
    configuration: &PartialConfiguration,
    since: Option<&str>,
) -> Result<Vec<OsString>, CliDiagnostic> {
    let default_branch = configuration
        .vcs
        .as_ref()
        .and_then(|v| v.default_branch.as_ref());

    let base = match (since, default_branch) {
        (Some(since), Some(_)) => since,
        (Some(since), None) => since,
        (None, Some(branch)) => branch,
        (None, None) => {
            return Err(CliDiagnostic::incompatible_end_configuration(
                "The `--changed` flag was set, but couldn't determine the base to compare against. Either set configuration.vcs.default_branch or use the --since argument.",
            ));
        }
    };

    let changed_files = fs.get_changed_files(base)?;

    let filtered_changed_files = changed_files.iter().map(OsString::from).collect::<Vec<_>>();

    Ok(filtered_changed_files)
}

pub(crate) fn get_staged_files(
    fs: &DynRef<'_, dyn FileSystem>,
) -> Result<Vec<OsString>, CliDiagnostic> {
    let staged_files = fs.get_staged_files()?;

    let filtered_staged_files = staged_files.iter().map(OsString::from).collect::<Vec<_>>();

    Ok(filtered_staged_files)
}
