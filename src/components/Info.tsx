import {
  Button,
  makeStyles,
  MessageBar,
  MessageBarActions,
  MessageBarBody,
  Text,
} from "@fluentui/react-components";
import { DismissRegular } from "@fluentui/react-icons";
import { useCallback, useEffect, useState } from "react";

const useStyles = makeStyles({
  info: {
    display: "flex",
    backgroundColor: "#111",
    color: "#888",
    pointerEvents: "none",
    position: "relative",
  },
  threadName: {
    marginLeft: "auto",
    marginRight: "auto",
    fontWeight: "bold",
    overflow: "hidden",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
  },
  resultWrapper: {
    position: "absolute",
    margin: "0 4px",
    bottom: 0,
    right: 0,
  },
  show: {
    animation: "200ms forwards",
    animationName: {
      "0%": {
        opacity: 0,
      },
      "100%": {
        opacity: 1,
      },
    },
  },
  hide: {
    animation: "500ms forwards",
    animationName: {
      "0%": {
        opacity: 1,
      },
      "100%": {
        opacity: 0,
      },
    },
  },
});

export default function Info(props: {
  threadName: string;
  message?: {
    intent: "success" | "error";
    text: string;
  } | null;
  onResize(): void;
}): JSX.Element {
  const classes = useStyles();
  const [show, setShow] = useState(true);

  useEffect(() => {
    props.onResize();
  }, [props.threadName]);
  useEffect(() => {
    if (props.message == null) {
      return;
    }
    setShow(true);
    setTimeout(() => {
      setShow(false);
    }, props.message.text.length * 500);
  }, [props.message]);

  const onClick = useCallback(() => {
    setShow(false);
  }, [setShow]);

  return (
    <div className={classes.info}>
      <Text className={classes.threadName} size={200} align="center">
        {props.threadName}
      </Text>
      <div className={classes.resultWrapper}>
        {props.message == null ? null : (
          <MessageBar
            className={`${show ? classes.show : classes.hide}`}
            shape="square"
            intent={props.message.intent}
          >
            <MessageBarBody>{props.message.text}</MessageBarBody>
            <MessageBarActions
              containerAction={
                <Button
                  aria-label="dismiss"
                  appearance="transparent"
                  icon={<DismissRegular />}
                  style={{ pointerEvents: show ? "auto" : "unset" }}
                  onClick={onClick}
                />
              }
            />
          </MessageBar>
        )}
      </div>
    </div>
  );
}
