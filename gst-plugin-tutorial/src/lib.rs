use gst::glib;

mod rgb2gray;
mod rsbasic;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    rsbasic::register(plugin)?;
    rgb2gray::register(plugin)?;
    Ok(())
}

// Static plugin metadata that is directly stored in the plugin shared object and read by GStreamer
// upon loading.
// Plugin name, plugin description, plugin entry point function, version number of this plugin,
// license of the plugin, source package name, binary package name, origin where it comes from
// and the date/time of release.
gst::plugin_define!(
    rstutorial,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
