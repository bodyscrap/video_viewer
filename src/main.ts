import { invoke, Channel } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';

// Canvas の設定
const canvas = document.getElementById("imageCanvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;
// ボタン
const openButton = document.getElementById("btnOpen") as HTMLButtonElement;
const playButton = document.getElementById("btnPlay") as HTMLButtonElement;
const stopButton = document.getElementById("btnStop") as HTMLButtonElement;

// 画像サイズ取得済みか
let isSizeReceived = false;

// 画像バッファの型
// 幅、高さ、24bitRGBバッファを持つ
type ImageBuffer =
{
  width: number;
  height: number;
  data: Uint8Array;
};

// フレームバッファ受信イベント
const onReceiveFrame = new Channel<ImageBuffer>();
onReceiveFrame.onmessage = (message) => {
  // canvas または ctx が null の場合は処理を行わない
  if (canvas === null || ctx === null) {
    return;
  }
  const { width, height, data } = message;

  // canvasのサイズをを画像のサイズに合わせる(初回のみ)
  if (!isSizeReceived) {
    canvas.width = width;
    canvas.height = height;
    isSizeReceived = true;
  }
  // ImageData の作成
  const imageData = ctx.createImageData(width, height);
  // RGB バッファを RGBA に変換
  for (let i = 0; i < data.length; i += 3) {
    const pixelIndex = (i / 3) * 4;
    imageData.data[pixelIndex] = data[i];         // R
    imageData.data[pixelIndex + 1] = data[i + 1]; // G
    imageData.data[pixelIndex + 2] = data[i + 2]; // B
    imageData.data[pixelIndex + 3] = 255;         // A (不透明)
  }
  // Canvas に描画
  ctx.putImageData(imageData, 0, 0);
};
// 動画を開く ボタンがクリックされたときの処理
openButton.addEventListener("click", async () => {
  open({
    multiple: false,
    filters: [
      { name: 'MP4', extensions: ['mp4'] },
    ],
  }).then(async file => {
    await invoke("init", { videoPath: file, onEvent:onReceiveFrame});
    isSizeReceived = false;
  });
});

// 再生 ボタンがクリックされたときの処理
playButton.addEventListener("click", async () => {
  await invoke("play");
});

// 停止 ボタンがクリックされたときの処理
stopButton.addEventListener("click", async () => {
  await invoke("stop");
});
