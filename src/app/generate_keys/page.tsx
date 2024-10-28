"use client";
import { useState } from "react";
import { Typography, Alert, Snackbar } from "@mui/material";
import { motion } from "framer-motion";
import { SyncLoader } from "react-spinners";
import { FaCopy } from "react-icons/fa";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { BiArrowToLeft } from "react-icons/bi";

export default function GenerateKeys() {
  const [loading, setLoading] = useState(false);
  const [keys, setKeys] = useState({ privateKey: "", publicKey: "" });
  const [error, setError] = useState("");
  const [notification, setNotification] = useState({ open: false, message: "" });
  const router = useRouter();

  const generateKeys = async () => {
    setLoading(true);
    setError("");
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<{ private: string; public: string }>("generate_keys");
      setKeys({
        privateKey: result.private,
        publicKey: result.public
      });
    } catch (e) {
      setError(e as string);
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = async (text: string, type: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setNotification({ open: true, message: `${type} copied to clipboard!` });
    } catch (err) {
      console.error("Failed to copy: ", err);
      setNotification({ open: true, message: "Failed to copy to clipboard" });
    }
  };

  const handleCloseNotification = () => {
    setNotification({ ...notification, open: false });
  };

  const handleNextClick = () => {
    setLoading(true);
    router.push(`/starter?private=${encodeURIComponent(keys.privateKey)}&public=${encodeURIComponent(keys.publicKey)}`);
  };

  return (
    <div className="flex flex-col items-center justify-center w-screen min-h-screen bg-gray-900 text-white p-6" onContextMenu={(e) => e.preventDefault()}>
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
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
        className="text-center mb-8"
      >
        <Typography variant="h5" className="text-emerald-300 mb-4">
          Generate Your Keys
        </Typography>
        <Typography variant="body1" className="text-gray-300">
          Click the button below to generate your private and public keys.
        </Typography>
      </motion.div>

      <motion.button
        whileHover={{ scale: 1.05, backgroundColor: "#34D399" }}
        whileTap={{ scale: 0.95 }}
        transition={{ type: "spring", stiffness: 400, damping: 10 }}
        className={`px-8 py-3 text-white rounded-full shadow-lg mb-8 ${
          keys.privateKey !== "" && keys.publicKey !== ""
            ? "bg-gray-500 cursor-not-allowed"
            : "bg-emerald-500 hover:bg-emerald-600"
        }`}
        onClick={generateKeys}
        disabled={keys.privateKey !== "" && keys.publicKey !== ""}
      >
        {keys.privateKey !== "" && keys.publicKey !== "" ? "Keys Generated" : "Generate Keys"}
      </motion.button>

      {keys.privateKey && keys.publicKey && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
          className="w-full max-w-md space-y-4"
        >
          <div className="bg-gray-800 p-4 rounded-md relative">
            <Typography variant="subtitle1" className="text-emerald-300 mb-2">
              Private Key:
            </Typography>
            <Typography variant="body2" className="text-gray-300 break-all">
              {keys.privateKey}
            </Typography>
            <button
              onClick={() => copyToClipboard(keys.privateKey, "Private Key")}
              className="absolute top-2 right-2 text-gray-400 hover:text-white"
            >
              <FaCopy />
            </button>
          </div>

          <div className="bg-gray-800 p-4 rounded-md relative">
            <Typography variant="subtitle1" className="text-emerald-300 mb-2">
              Public Key (Wallet Address):
            </Typography>
            <Typography variant="body2" className="text-gray-300 break-all">
              {keys.publicKey}
            </Typography>
            <button
              onClick={() => copyToClipboard(keys.publicKey, "Public Key")}
              className="absolute top-2 right-2 text-gray-400 hover:text-white"
            >
              <FaCopy />
            </button>
          </div>

          <motion.button
            whileHover={{ scale: 1.05, backgroundColor: "#34D399" }}
            whileTap={{ scale: 0.95 }}
            transition={{ type: "spring", stiffness: 400, damping: 10 }}
            className="w-full py-3 bg-emerald-500 text-white rounded-full shadow-lg mt-4"
            onClick={handleNextClick}
            disabled={loading}
          >
            {loading ? <SyncLoader size={8} color="#FFFFFF" /> : "Next"}
          </motion.button>
        </motion.div>
      )}

      {error && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="mt-4"
        >
          <Alert severity="error" className="bg-red-900 text-white">
            {error}
          </Alert>
        </motion.div>
      )}

      <Snackbar
        open={notification.open}
        autoHideDuration={3000}
        onClose={handleCloseNotification}
        message={notification.message}
      />
    </div>
  );
}
