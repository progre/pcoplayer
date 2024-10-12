import { Button, makeStyles, Textarea } from "@fluentui/react-components";
import { Send16Filled as Send } from "@fluentui/react-icons";
import { MouseEvent, useCallback, useEffect, useRef, useState } from "react";

const isMac = window.navigator.userAgent.includes("Mac OS X");

const useStyles = makeStyles({
  commentWrapper: {
    display: "flex",
    backgroundColor: "#111",
    padding: "4px",
  },
  comment: {
    minWidth: 0,
    width: "100%",
    backgroundColor: "#111",
    marginRight: "4px",
    borderRadius: 0,
    "& > textarea": {
      minHeight: "0",
      margin: 0,
      padding: 0,
    },
    "&::after": {
      borderRadius: "none",
    },
  },
  post: {
    "&:disabled": {
      color: "#222",
      pointerEvents: "none",
    },
  },
});

export default function Comment(props: {
  textareaRef: React.RefObject<HTMLTextAreaElement>;
  onResize(): void;
  onClickPost(comment: string): void;
}): JSX.Element {
  const [comment, setComment] = useState<string>("");
  const buttonRef = useRef<HTMLButtonElement>(null);

  const classes = useStyles();

  useEffect(() => {
    props.onResize();
  }, [comment.split("\n").length]);

  return (
    <div
      className={classes.commentWrapper}
      title={`${isMac ? "⌘ + return" : "Ctrl + Enter"} で書き込み`}
      onKeyDown={(ev) => {
        if (
          (isMac && ev.metaKey && ev.key === "Enter") ||
          (!isMac && ev.ctrlKey && ev.key === "Enter")
        ) {
          buttonRef.current!.click();
          return;
        }
        if (ev.key === "Escape") {
          props.textareaRef.current!.blur();
          return;
        }
      }}
    >
      <Textarea
        className={classes.comment}
        appearance="filled-darker"
        value={comment}
        ref={props.textareaRef}
        onChange={(ev) => {
          setComment(ev.currentTarget.value);
        }}
        rows={comment.split("\n").length}
      />
      <Button
        className={classes.post}
        ref={buttonRef}
        disabled={comment.length === 0}
        appearance="primary"
        size="small"
        shape="square"
        onClick={useCallback(
          async (ev: MouseEvent<HTMLButtonElement>) => {
            ev.currentTarget.focus();
            setComment("");
            props.onClickPost(comment);
          },
          [comment, setComment]
        )}
      >
        <Send />
      </Button>
    </div>
  );
}
