use std::str::FromStr;

use gstreamer::{
    self,
    prelude::{ElementExt, GstBinExtManual},
};

fn main() {
    let res = gstreamer::init();
    if res.is_err() {
        return;
    }

    let source: gstreamer::Element = gstreamer::ElementFactory::make("autovideosrc")
        .build()
        .unwrap();
    let videoconvert: gstreamer::Element = gstreamer::ElementFactory::make("videoconvert")
        .build()
        .unwrap();
    let videoscale: gstreamer::Element = gstreamer::ElementFactory::make("videoscale")
        .build()
        .unwrap();
    let caps: gstreamer::Caps =
        gstreamer::Caps::from_str("video/x-raw,width=640,height=480").unwrap();
    let capsfilter: gstreamer::Element = gstreamer::ElementFactory::make("capsfilter")
        .property("caps", caps)
        .build()
        .unwrap();

    let encoder: gstreamer::Element = gstreamer::ElementFactory::make("theoraenc")
        .build()
        .unwrap();
    let muxer: gstreamer::Element = gstreamer::ElementFactory::make("oggmux").build().unwrap();
    let sink: gstreamer::Element = gstreamer::ElementFactory::make("tcpserversink")
        .property("host", "127.0.0.1")
        .property("port", 8181)
        .build()
        .unwrap();

    let pipeline: gstreamer::Pipeline = gstreamer::Pipeline::with_name("test-pipeline");
    pipeline
        .add_many([
            &source,
            &videoconvert,
            &videoscale,
            &capsfilter,
            &encoder,
            &muxer,
            &sink,
        ])
        .unwrap();
    gstreamer::Element::link_many([
        &source,
        &videoconvert,
        &videoscale,
        &capsfilter,
        &encoder,
        &muxer,
        &sink,
    ])
    .unwrap();
    pipeline.set_state(gstreamer::State::Playing).unwrap();

    // Wait until error or EOS
    let bus: gstreamer::Bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(_err) => {
                println!("Error",);
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    println!("Hello, world!");
}
