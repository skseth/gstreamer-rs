// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use gst::glib;
use gst::subclass::prelude::*;
use gst_base::subclass::prelude::*;
use once_cell::sync::Lazy;

// Struct containing all the element data
#[derive(Default)]
pub struct RsBasic {}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for RsBasic {
    const NAME: &'static str = "RsBasic";
    type Type = super::RsBasic;
    type ParentType = gst_base::BaseTransform;
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for RsBasic {}

impl GstObjectImpl for RsBasic {}

// Implementation of gst::Element virtual methods
impl ElementImpl for RsBasic {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Rust Basic",
                "Simplest Element",
                "The simplest element which compiles",
                "Samir Seth<samirkseth>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}

// Implementation of gst_base::BaseTransform virtual methods
impl BaseTransformImpl for RsBasic {
    // Configure basetransform so that we are never running in-place,
    // don't passthrough on same caps and also never call transform_ip
    // in passthrough mode (which does not matter for us here).
    //
    const MODE: gst_base::subclass::BaseTransformMode =
        gst_base::subclass::BaseTransformMode::NeverInPlace;
    const PASSTHROUGH_ON_SAME_CAPS: bool = false;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;
}
