# Tauri + GStreamer で動画再生

## 1. 概要

GStreamer のRustバインディングでアプリケーションを作りたい。  
ということで、Tauri と組み合わせて動画再生アプリケーションを作成しました。  

GStreamerのAppSinkを使って、フレームを読み込み、Channelを使ってTauriのフロントエンド側に画像サイズとバッファ(24bit RGB)を送信するようにしています。  

## 2. 直近見えている課題

**表示更新が遅い**  
GStreamer側ではフレーム取得時に、コンソールにメッセージを出力しています。  
それを見る限りは、フレーム取得自体は早いように見えます。  
おそらく受け取ったバッファを元にcanvasに描画する処理が遅いのだと思われます。  

ここはTypeScriptなのかHTMLの仕様周りなのかもう少し調べる必要があります。  

というか、Tauriのボトルネック解析どうするん?