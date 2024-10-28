"use client";
import { Typography } from "@mui/material";
import { motion } from "framer-motion";
import { GiHouseKeys } from "react-icons/gi";
import { useRouter } from "next/navigation";
import { SyncLoader } from "react-spinners";
import { useState } from "react";

export default function Keys() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);

  const handleHaveBtn = () => {
    setLoading(true);
    router.push("/key_check");
  };

  const handleGenerateBtn = () => {
    setLoading(true);
    router.push("/generate_keys");
  };

  const text =
    "To become a Validator on the Centichain network, you'll need a unique keypair. If you already have one, click 'I HAVE'. If not, don't worry – you can quickly generate a new keypair by clicking 'GENERATE'. This keypair is essential for your role in securing and validating transactions on our decentralized network.";

  return (
    <div className="flex flex-col items-center justify-center w-screen h-screen bg-gray-900 text-white p-6" onContextMenu={(e) => e.preventDefault()}>
      <motion.div
        initial={{ scale: 0 }}
        animate={{ scale: 1 }}
        transition={{ type: "spring", stiffness: 260, damping: 20 }}
      >
        <GiHouseKeys className="text-6xl text-emerald-400 mb-8" />
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
        className="max-w-md text-center mb-12"
      >
        <Typography variant="body1" className="text-gray-200">
          {text}
        </Typography>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.2 }}
        className="space-y-6 flex flex-col items-center"
      >
        <motion.button
          whileHover={{ scale: 1.05, backgroundColor: "#34D399" }}
          whileTap={{ scale: 0.95 }}
          transition={{ type: "spring", stiffness: 400, damping: 10 }}
          className="w-48 py-3 bg-emerald-500 text-white rounded-full shadow-lg"
          onClick={handleHaveBtn}
        >
          I HAVE
        </motion.button>
        
        <motion.button
          whileHover={{ scale: 1.05, backgroundColor: "#4B5563" }}
          whileTap={{ scale: 0.95 }}
          transition={{ type: "spring", stiffness: 400, damping: 10 }}
          className="w-48 py-3 bg-gray-700 text-white rounded-full shadow-lg"
          onClick={handleGenerateBtn}
        >
          GENERATE
        </motion.button>
      </motion.div>

      {loading && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.5 }}
          className="mt-8"
        >
          <SyncLoader size={8} color="#10B981" />
        </motion.div>
      )}
    </div>
  );
}
