#!/bin/bash

GST_PLUGIN_PATH=$(pwd)/target/debug
export GST_PLUGIN_PATH

gst-inspect-1.0 "$GST_PLUGIN_PATH/libgstrstutorial.dylib"

gst-launch-1.0 videotestsrc ! rsrgb2gray ! videoconvert ! autovideosink
