import { makeStyles } from "@fluentui/react-components";
import "./App.css";
import Info from "./components/Info";
import Comment from "./components/Comment";

const useStyles = makeStyles({
  container: {
    display: "flex",
    flexDirection: "column",
    width: "100%",
    height: "100%",
  },
  video: {
    backgroundColor: "#000",
  },
});

export default function App(props: {
  mainRef: React.RefObject<HTMLDivElement>;
  videoRef: React.RefObject<HTMLVideoElement>;
  interfaceRef: React.RefObject<HTMLDivElement>;
  textareaRef: React.RefObject<HTMLTextAreaElement>;
  threadName: string;
  message?: {
    intent: "success" | "error";
    text: string;
  } | null;
  onResizeVideo(videoWidth: number, videoHeight: number): void;
  onResizeFrame(): void;
  onMouseDownVideo(ev: React.MouseEvent): void;
  onClickPost(comment: string): void;
}): JSX.Element {
  const classes = useStyles();

  return (
    <main className={classes.container} ref={props.mainRef}>
      <video
        className={classes.video}
        ref={props.videoRef}
        onMouseDown={props.onMouseDownVideo}
        onResize={(ev) => {
          const target = ev.currentTarget;
          props.onResizeVideo(target.videoWidth, target.videoHeight);
        }}
      />
      <div ref={props.interfaceRef}>
        <Info
          threadName={props.threadName}
          message={props.message}
          onResize={props.onResizeFrame}
        />
        <Comment
          textareaRef={props.textareaRef}
          onResize={props.onResizeFrame}
          onClickPost={props.onClickPost}
        />
      </div>
    </main>
  );
}
