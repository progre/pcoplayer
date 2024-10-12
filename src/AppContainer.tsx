import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import mpegts from "mpegts.js";
import { useEffect, useRef, useState } from "react";
import App from "./App";

async function initPlayer(
  video: HTMLVideoElement,
  url: string
): Promise<mpegts.Player | null> {
  const player = mpegts.createPlayer({
    type: "mse", // could also be mpegts, m2ts, flv
    isLive: true,
    url,
  });
  player.attachMediaElement(video);
  player.load();
  return player;
}

function deinitPlayer(player: mpegts.Player): void {
  player.unload();
  player.detachMediaElement();
}

async function refreshPlayer(
  video: HTMLVideoElement,
  player: mpegts.Player | null,
  url: string
): Promise<mpegts.Player | null> {
  if (player != null) {
    deinitPlayer(player);
  }
  return initPlayer(video, url);
}

export default function AppContaier(): JSX.Element {
  const mainRef = useRef<HTMLDivElement>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const interfaceRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [player, setPlayer] = useState<mpegts.Player | null>(null);
  const [threadName, setThreadName] = useState<string>("");
  const [bbs, setBbs] = useState<{ url: string; charset: string } | null>(null);
  const [message, setMessage] = useState<{
    intent: "success" | "error";
    text: string;
  } | null>(null);

  useEffect(() => {
    const video = videoRef.current!;
    video.autoplay = true;

    (async () => {
      let initialData: any = await invoke("initialize", {
        innerWidth: window.innerWidth * window.devicePixelRatio,
        innerHeight: window.innerHeight * window.devicePixelRatio,
        videoClientWidth: video.clientWidth,
        videoClientHeight: video.clientHeight,
      });
      setPlayer(await refreshPlayer(video, player, initialData.url));
    })();
  }, []);

  useEffect(() => {
    const handlePaste = async (e: ClipboardEvent) => {
      if (e.target === textareaRef.current) {
        return;
      }
      const url = e.clipboardData?.getData("text");
      if (url == null) {
        return;
      }
      const result = (await invoke("resolve_url", { url })) as any;
      if (result == null) {
        return;
      }
      if (result.type === "stream") {
        setPlayer(
          await refreshPlayer(videoRef.current!, player, result.streamUrl)
        );
      } else if (result.type === "bbs") {
        setThreadName(result.threadName);
        setBbs({ url: result.threadUrl, charset: result.charset });
      }
    };
    document.addEventListener("paste", handlePaste);

    return () => {
      if (player != null) {
        deinitPlayer(player);
      }
      document.removeEventListener("paste", handlePaste);
    };
  }, [player]);

  return (
    <App
      mainRef={mainRef}
      videoRef={videoRef}
      interfaceRef={interfaceRef}
      textareaRef={textareaRef}
      threadName={threadName}
      message={message}
      onMouseDownVideo={async (_ev) => {
        await getCurrentWindow().startDragging();
      }}
      onResizeVideo={async (videoWidth, videoHeight) => {
        await invoke("resize_video", {
          width: videoWidth,
          height: videoHeight,
        });
      }}
      onResizeFrame={async () => {
        await invoke("resize_interface", {
          interfaceHeight: Math.round(
            interfaceRef.current!.clientHeight * window.devicePixelRatio
          ),
        });
      }}
      onClickPost={async (comment: string) => {
        const url = bbs?.url;
        const charset = bbs?.charset;
        const name = "";
        const email = "sage";
        const msg = comment;
        try {
          await invoke("post", { url, charset, name, email, msg });
          setMessage({
            intent: "success",
            text: "書き込みました",
          });
        } catch (e) {
          setMessage({
            intent: "error",
            text: e as string,
          });
        }
      }}
    />
  );
}
