"use client";
import { useEffect, useState } from "react";
import { Typography } from "@mui/material";
import dynamic from 'next/dynamic';

const Keys = dynamic(() => import("../components/keys"), { ssr: false });
const SyncLoader = dynamic(() => import("react-spinners").then(mod => mod.SyncLoader), { ssr: false });

export default function Starter() {
  const [memCheck, setMemCheck] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Move the Tauri invoke call inside useEffect
    const checkMemory = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const result = await invoke<boolean>("memory_check");
        setLoading(false);
        setMemCheck(result);
      } catch (e) {
        setLoading(false);
        console.log(e)
      }
    };
    checkMemory();
  }, []);

  return (
    <div
      className="w-full h-screen flex justify-center items-center select-none"
      onContextMenu={(e) => e.preventDefault()}
    >
      {memCheck && !loading ? (
        <Keys />
      ) : !memCheck && !loading ? (
        <Typography variant="h5" color="red">
          {"You Dont' have Enough Memory For Join To Centichain!"}
        </Typography>
      ) : (
        <SyncLoader color="gray" size={5} />
      )}
    </div>
  );
}
