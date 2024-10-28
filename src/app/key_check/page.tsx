"use client";
import { Typography, Alert } from "@mui/material";
import { useState } from "react";
import Link from "next/link";
import { motion } from "framer-motion";
import { BiArrowToLeft } from "react-icons/bi";
import { FaPaste } from "react-icons/fa";
import { SyncLoader } from "react-spinners";
import { readText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "next/navigation";

interface Response {
  key: boolean;
  status: string;
}

export default function CheckKey() {
  const [pkey, setPkey] = useState("");
  const [isErr, setIsErr] = useState(false);
  const [res, setRes] = useState("");
  const [loading, setLoading] = useState(false);
  const router = useRouter();

  const check = () => {
    setLoading(true);
    invoke<Response>("check_key", { pkey: pkey }).then((result) => {
      if (result.key) {
        router.push(`/starter?private=${pkey}&public=${result.status}`);
      } else {
        setLoading(false);
        setIsErr(true);
        setRes(result.status);
      }
    });
  };

  const handlePaste = async () => {
    try {
      setRes("");
      const copied = await readText();
      if (copied) {
        setPkey(copied.toString());
      } else {
        setIsErr(true);
        setRes("No text found in clipboard");
      }
    } catch (error) {
      setIsErr(true);
      setRes("Failed to paste from clipboard: " + error);
    }
  };

  return (
    <div
      className="flex flex-col items-center justify-center w-screen h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white p-6"
      onContextMenu={(e) => e.preventDefault()}
    >
      <motion.div
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
        className="absolute top-4 left-4"
      >
        <Link href="/pre_start">
          <motion.div
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
            className="cursor-pointer text-slate-300 w-10 h-10 flex justify-center items-center rounded-full bg-gray-800 hover:bg-gray-700 transition duration-200"
          >
            <BiArrowToLeft size={25} />
          </motion.div>
        </Link>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
        className="max-w-md text-center mb-8"
      >
        <Typography variant="h5" className="text-emerald-300 mb-4">
          Enter Your Private Key
        </Typography>
        <Typography variant="body1" className="text-gray-300">
          Please enter your private key to begin validating as a validator. You
          can paste the private key using the paste button.
        </Typography>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.2 }}
        className="w-full max-w-md"
      >
        <div className="relative">
          <textarea
            value={pkey}
            onChange={(e) => {
              setRes("");
              setPkey(e.target.value);
            }}
            className="w-full p-3 bg-gray-800 text-white border border-gray-700 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500 transition duration-200"
            style={{ resize: "none", height: "auto", overflow: "hidden" }}
            spellCheck={false}
            rows={4}
            onInput={(e) => {
              e.currentTarget.style.height = "auto";
              e.currentTarget.style.height =
                e.currentTarget.scrollHeight + "px";
            }}
          />
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="absolute right-2 top-2 p-2 bg-gray-700 rounded-md text-white hover:bg-gray-600 transition duration-200"
            onClick={handlePaste}
          >
            <FaPaste size={20} />
          </motion.button>
        </div>
      </motion.div>

      <motion.button
        whileHover={{ scale: 1.05, backgroundColor: "#34D399" }}
        whileTap={{ scale: 0.95 }}
        transition={{ type: "spring", stiffness: 400, damping: 10 }}
        className="mt-8 px-8 py-3 bg-emerald-500 text-white rounded-full shadow-lg"
        onClick={check}
      >
        Check Key
      </motion.button>

      <div className="mt-8 h-16">
        {isErr && res ? (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <Alert severity="error" className="bg-red-900 text-white">
              {res}
            </Alert>
          </motion.div>
        ) : loading ? (
          <SyncLoader size={8} color="#10B981" />
        ) : null}
      </div>
    </div>
  );
}
