"use client";
import { Col, Row } from "antd";
import { BackBtn } from "@/app/components/tools/components/button";
import { Typography, CircularProgress, LinearProgress } from "@mui/material";
import { BsCheckCircle, BsDownload } from "react-icons/bs";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// Main component for displaying Updates
export default function Update({
  setShow,
}: {
  setShow: (show: boolean) => void;
}) {
  const [updateStatus, setUpdateStatus] = useState<string>(
    "Checking for updates..."
  );
  const [updateProgress, setUpdateProgress] = useState<number>(0);
  const [updatesAvailable, setUpdatesAvailable] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const back = () => {
    setShow(false);
  };

  useEffect(() => {
    setIsLoading(true);

    invoke("check_for_updates");

    listen("available", (event) => {
      setUpdatesAvailable(event.payload as boolean);
      if ((event.payload as boolean) === false) {
        setIsLoading(false);
        setUpdateStatus("Your app is up to date.");
      }
    });

    listen("progress", (event) => {
      setUpdateProgress(event.payload as number);
    });

    listen("status", (event) => {
      setUpdateStatus(event.payload as string);
      setIsLoading(false);
    });
  }, []);

  return (
    <div className="w-full p-2 overflow-hidden">
      {/* Header with back button and Update title */}
      <Row className="h-[50px]">
        <Col span={11}>
          <div className="w-[40px] h-[40px] rounded-md">
            <BackBtn onClick={back} />
          </div>
        </Col>

        <Col span={13}>
          <div className="w-full flex items-center justify-center bg-gradient-to-r from-slate-800 to-slate-900 rounded-lg rounded-br-none p-1 shadow-lg">
            <Typography
              variant="h4"
              className="text-white font-bold tracking-wide"
            >
              Updates
            </Typography>
          </div>
        </Col>
      </Row>

      {/* Display updates or up-to-date message */}
      <Row gutter={[0, 16]}>
        <Col span={24}>
          <div className="w-full h-[300px] flex flex-col items-center justify-center bg-gradient-to-b from-slate-800 to-slate-900 rounded-lg shadow-lg">
            {isLoading ? (
              <CircularProgress />
            ) : updatesAvailable ? (
              <>
                <BsDownload size={60} className="text-blue-400 mb-4" />
                <Typography variant="h5" className="text-gray-300 mb-2">
                  {updateStatus}
                </Typography>
                <div className="w-3/4 mt-4">
                  <LinearProgress
                    variant="determinate"
                    value={updateProgress}
                  />
                </div>
              </>
            ) : (
              <>
                <BsCheckCircle size={60} className="text-green-400 mb-4" />
                <Typography variant="h5" className="text-gray-300 mb-2">
                  Your App is Up to Date
                </Typography>
                <Typography
                  variant="body2"
                  className="text-gray-500 text-center max-w-md"
                >
                  You&apos;re running the latest version of the application. We
                  regularly release updates to improve performance, add new
                  features, and fix bugs. Check back here periodically to ensure
                  you&apos;re always using the most up-to-date version.
                </Typography>
              </>
            )}
          </div>
        </Col>
      </Row>
    </div>
  );
}
