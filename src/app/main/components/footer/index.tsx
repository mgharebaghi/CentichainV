"use client";
import { Typography } from "@mui/material";
import { listen } from "@tauri-apps/api/event";
import { Col, Row } from "antd";
import { useEffect, useState } from "react";
import { BsPersonRaisedHand } from "react-icons/bs";

export default function Footer(props: { turn: string; leader: string, setLeader: (leader: string) => void }) {
  const [err, setErr] = useState("");
  const [status, setStatus] = useState("");

  useEffect(() => {
    // Listen for error events and update state
    listen<string>("error", (event) => {
      setStatus("");
      setErr(event.payload);
    });

    // Listen for status events and update state
    listen<string>("status", (event) => {
      setErr("");
      setStatus(event.payload);
    });

    listen<string>("leader", (event) => {
      props.setLeader(event.payload);
    });
  }, []);

  return (
    <Row className="w-full h-[45px] flex items-center justify-between overflow-hidden">
      <Col span={12} className="h-full overflow-hidden flex items-center bg-gray-800 rounded-md px-3">
        <BsPersonRaisedHand
          size={20}
          color={props.turn === "true" ? "#70be00" : ""}
          className="mr-2"
        />
        <Typography noWrap variant="body2" className="text-slate-300">
          Leader: {props.turn === "true" ? "You" : props.leader}
        </Typography>
      </Col>

      <Col span={12} className="h-full flex items-center justify-center overflow-hidden bg-gray-700 rounded-md px-3">
        {status ? (
          <Typography noWrap variant="body2" className="text-slate-300">
            {status}
          </Typography>
        ) : err ? (
          <Typography noWrap variant="body2" className="text-red-400">
            {err}
          </Typography>
        ) : (
          <Typography noWrap variant="body2" className="text-gray-400">
            No messages available
          </Typography>
        )}
      </Col>
    </Row>
  );
}
