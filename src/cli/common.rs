/// Common help text for environment variables used by multiple CLI commands.
pub const ENV_HELP: &str = "\
Environment:
  OMAKURE_SCRIPTS_DIR       Scripts directory override
  OVERTURE_SCRIPTS_DIR      Legacy scripts directory override
  CLOUD_MGMT_SCRIPTS_DIR    Legacy scripts directory override";

/// Extended environment help including repo variables.
pub const ENV_HELP_WITH_REPO: &str = "\
Environment:
  REPO                      GitHub repository (same as --repo)
  VERSION                   Release tag (same as --version)
  OMAKURE_REPO              Override repo without clobbering REPO
  OMAKURE_SCRIPTS_DIR       Scripts directory override
  OVERTURE_REPO             Legacy repo override
  OVERTURE_SCRIPTS_DIR      Legacy scripts directory override
  CLOUD_MGMT_REPO           Legacy repo override
  CLOUD_MGMT_SCRIPTS_DIR    Legacy scripts directory override";

/// Check if help flag is present in args.
pub fn wants_help(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-h" || arg == "--help")
}
