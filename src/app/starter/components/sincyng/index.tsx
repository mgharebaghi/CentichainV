"use client";
import React, { useEffect, useState } from "react";
import { Typography } from "@mui/material";
import { SyncLoader } from "react-spinners";
import { useRouter } from "next/navigation";
import { FcOk } from "react-icons/fc";
import { Progress } from "antd";
import { motion, AnimatePresence } from "framer-motion";
import { listen } from "@tauri-apps/api/event";

interface SyncingProps {
  publicKey: string;
  relay: string;
  status: string;
  err: string;
  loading: boolean;
  turnGot: boolean;
  privateKey: string;
  turn: string;
  dlPercent: number;
  setTurn: any;
  setTurnGot: any;
  peerId: string;
}

export default function Syncing({
  publicKey,
  relay,
  status,
  err,
  loading,
  turnGot,
  privateKey,
  turn,
  dlPercent,
  setTurn,
  setTurnGot,
  peerId,
}: SyncingProps) {
  const router = useRouter();
  const [message, setMessage] = useState("Downloading MongoDB...");

  useEffect(() => {
    if (typeof window !== "undefined" && status.includes("Sync message sent") && turnGot) {
      router.push(`/main?private=${privateKey}&public=${publicKey}&relay=${relay}&status=${status}&err=${err}&turn=${turn}&peerId=${peerId}`);
    }
  }, [status, turnGot, privateKey, publicKey, relay, err, turn, router]);

  useEffect(() => {
    listen<string>("turn", (event) => {
      setTurn(event.payload);
      setTurnGot(true);
    });

    const messages = [
      "Powering Centichain with scalable document storage...",
      "Leveraging MongoDB's flexible schema for blockchain data...",
      "Enhancing Centichain performance with MongoDB...",
      "Ensuring data integrity with MongoDB's ACID transactions...",
      "Downloading MongoDB...",
    ];
    const interval = setInterval(() => {
      setMessage(messages[Math.floor(Math.random() * messages.length)]);
    }, 12000);

    return () => clearInterval(interval);
  }, []);

  const renderStatus = () => {
    if (dlPercent < 100 && err === "" && relay === "" && status !== "installed" && status !== "downloaded") {
      return (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.5 }}
          className="w-full max-w-md"
        >
          <Typography variant="body1" className="mb-2 text-gray-300">
            <AnimatePresence mode="wait">
              <motion.span
                key={message}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -20 }}
                transition={{ duration: 0.5 }}
              >
                {message}
              </motion.span>
            </AnimatePresence>
          </Typography>
          <Progress
            percent={dlPercent}
            status="active"
            strokeColor="#10B981"
            trailColor="#1F2937"
            format={(percent) => (
              <span className="text-gray-400">{percent}%</span>
            )}
          />
        </motion.div>
      );
    }

    if (status.includes("Sync message sent")) {
      return (
        <motion.div
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5 }}
          className="flex items-center text-green-400"
        >
          <FcOk size={24} className="mr-2" />
          <span>{status}</span>
        </motion.div>
      );
    }

    return <Typography className="text-gray-300">{status}</Typography>;
  };

  return (
    <div className="w-full h-screen flex flex-col justify-center items-center bg-gray-900 text-white p-4">
      <motion.div
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
        className="mb-8 p-6 bg-gray-800 rounded-lg shadow-lg"
      >
        <Typography variant="h6" className="mb-2 text-emerald-300">
          Your public key
        </Typography>
        <code className="font-mono bg-gray-700 text-emerald-100 p-3 rounded-md text-sm block overflow-hidden text-ellipsis">
          {publicKey}
        </code>
      </motion.div>

      <div className="w-full max-w-md flex flex-col items-center justify-center">
        {relay && (
          <Typography variant="subtitle1" className="mb-4 text-emerald-400">
            Connected to: <span className="font-semibold">{relay}</span>
          </Typography>
        )}
        {renderStatus()}
      </div>

      {err && (
        <Typography className="text-red-400 mt-4 text-center">{err}</Typography>
      )}

      {loading && !err && dlPercent === 0 && (
        <div className="mt-6">
          <SyncLoader size={5} color="#10B981" />
        </div>
      )}
    </div>
  );
}
