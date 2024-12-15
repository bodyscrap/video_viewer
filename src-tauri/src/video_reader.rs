use tauri::ipc::Channel;
use anyhow::Result;
use gst;
use gst::Element;
use gst_video::VideoInfo;
use gst::prelude::*;
use gst_app;
use gst_app::{AppSink, AppSinkCallbacks};
use serde::Serialize;

/// GStreamerによる動画再生アプリ
#[derive(Default)]
pub struct VideoReader {
    /// パイプライン
    pipeline: Option<Element>,
}

#[derive(Clone, Serialize)]
pub struct ImageBuffer {
    width : i32,
    height : i32,
    data: Vec<u8>,
}

impl VideoReader{
    /// 動画ファイルを開き再生準備をする
    pub fn init(&mut self, video_path: &str, on_event:Channel<ImageBuffer>) -> Result<(), Box<dyn std::error::Error>>
    {
        // video_path中の\\を/に変換
        let video_path = video_path.replace("\\", "/");
        // 動画再生の停止
        self.stop()?;
        // パイプラインを作成
        let pipeline = gst::parse::launch(&format!(
            "filesrc location={} ! decodebin ! videoconvert ! videoscale ! video/x-raw,format=RGB ! appsink name=sink",
            video_path
        ))?;
        // AppSinkの取得
        let appsink = pipeline
            .clone()
            .dynamic_cast::<gst::Pipeline>()
            .expect("Pipeline cast failed")
            .by_name("sink")
            .expect("Sink not found")
            .dynamic_cast::<AppSink>()
            .expect("Sink element is not an AppSink");
        // AppSinkのnew_sampleイベントにコールバックを設定
        appsink.set_callbacks(
            AppSinkCallbacks::builder()
                .new_sample(move |appsink| { // サンプル(フレーム)取得時のコールバック
                    println!("new_sample");
                    // サンプルの取得
                    let sample = appsink.pull_sample().expect("Failed to pull sample");
                    // サンプルからフレームの情報を取得
                    let caps = sample.caps().expect("Failed to get caps");
                    let video_info = VideoInfo::from_caps(&caps).expect("Failed to get video info");
                    // サンプルからフレームのバッファを取得
                    let buffer = sample.buffer().expect("Failed to get buffer");
                    let map = buffer.map_readable().expect("Failed to map buffer");
                    let data = map.as_slice();
                    // 画像サイズとバッファをペイロードにまとめる
                    let payload = ImageBuffer {
                        width: video_info.width() as i32,
                        height: video_info.height() as i32,
                        data: data.to_vec(),
                    };
                    on_event.send(payload).expect("send failed");
                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );
        self.pipeline = Some(pipeline);
        println!("VideoReader initialized");
        Ok(())
    }

    /// 動画再生を開始する
    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if let Some(pipeline) = &self.pipeline {
            pipeline.set_state(gst::State::Playing)?;
        }
        Ok(())
    }

    /// 動画再生を停止する
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        if let Some(pipeline) = &self.pipeline {
            pipeline.set_state(gst::State::Null)?;
            self.pipeline = None;
        }
        Ok(())
    }
}
