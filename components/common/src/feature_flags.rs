//! All the feature flags that are recogized by Habitat.
//!
//! In general, feature flags are enabled by setting the corresponding
//! environment variable.
//!
//! Your binary *must* call `enable_features_from_env` for the flags
//! to be set appropriately.
//!
//! To add a new feature flag, you will need to add the bit mask
//! constant in the `features!` block below, as well as a mapping from
//! the feature to the environment variable to which it corresponds in
//! the `ENV_VARS` map below.

use crate::ui::{UIWriter,
                UI};
use features::features;
use habitat_core::env as henv;
use lazy_static::lazy_static;
use std::{collections::HashMap,
          iter::FromIterator};

// TODO (CM): It would be nice to come up with a way to more
// programmatically manage these flags. It's a bit of a pain to define
// the flag, and then define the environment variables
// separately. Nothing statically guarantees that you've specified an
// variable for a flag.

// TODO (CM): It'd be great to have a built-in way to document them,
// too.

// TODO (CM): Part of that documentation might be *when* a flag was
// added. In general, long-lived flags are a code-smell.

// TODO (CM): It may also be useful to break out features by area of
// concern. The `features` crate allows us to define multiple sets of
// feature flags and deal with them separately.

features! {
    pub mod feat {
        const List           = 0b0000_0000_0001,
        const TestExit       = 0b0000_0000_0010,
        const TestBootFail   = 0b0000_0000_0100,
        const RedactHTTP     = 0b0000_0000_1000,
        const IgnoreSignals  = 0b0000_0001_0000,
        const InstallHook    = 0b0000_0010_0000,
        const OfflineInstall = 0b0000_0100_0000,
        const IgnoreLocal    = 0b0000_1000_0000,
        const EventStream    = 0b0001_0000_0000
    }
}

lazy_static! {
    static ref ENV_VARS: HashMap<feat::Flags, &'static str> = {
        let mapping = vec![(feat::List, "HAB_FEAT_LIST"),
                           (feat::TestExit, "HAB_FEAT_TEST_EXIT"),
                           (feat::TestBootFail, "HAB_FEAT_BOOT_FAIL"),
                           (feat::RedactHTTP, "HAB_FEAT_REDACT_HTTP"),
                           (feat::IgnoreSignals, "HAB_FEAT_IGNORE_SIGNALS"),
                           (feat::InstallHook, "HAB_FEAT_INSTALL_HOOK"),
                           (feat::OfflineInstall, "HAB_FEAT_OFFLINE_INSTALL"),
                           (feat::IgnoreLocal, "HAB_FEAT_IGNORE_LOCAL"),
                           (feat::EventStream, "HAB_FEAT_EVENT_STREAM")];
        HashMap::from_iter(mapping)
    };
}

/// If the environment variable for a flag is set to _anything_ but
/// the empty string, it is activated.
pub fn enable_features_from_env(ui: &mut UI) {
    for (feature, env_var) in ENV_VARS.iter() {
        if henv::var(env_var).is_ok() {
            feat::enable(*feature);
            ui.warn(&format!("Enabling feature: {:?}", feature))
              .unwrap();
        }
    }

    // TODO (CM): Once the other TODOs above are done (especially the
    // documentation bits), it would be nice to extract this logic
    // into an actual discoverable CLI subcommand; it's a little weird
    // that you have to know how to enable a feature flag before you
    // can even find out that there *are* feature flags to enable.
    //
    // There's no reason why "list feature flags" should itself be a
    // feature-flag.
    if feat::is_enabled(feat::List) {
        ui.warn("Listing feature flags environment variables:")
          .unwrap();
        for (feature, env_var) in ENV_VARS.iter() {
            ui.warn(&format!("  * {:?}: {}={}",
                             feature,
                             env_var,
                             henv::var(env_var).unwrap_or_default()))
              .unwrap();
        }
    }
}
