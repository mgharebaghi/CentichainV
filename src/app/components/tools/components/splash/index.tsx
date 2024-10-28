"use client";
import { motion } from "framer-motion";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

function SplashScreen() {
  const router = useRouter();
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const timer = setTimeout(() => {
      router.push("/pre_start");
    }, 5000);

    const interval = setInterval(() => {
      setProgress((prevProgress) => Math.min(prevProgress + 1, 100));
    }, 50);

    return () => {
      clearTimeout(timer);
      clearInterval(interval);
    };
  }, [router]);

  return (
    <div className="w-screen h-screen flex flex-col items-center justify-center bg-gradient-to-br from-gray-900 to-gray-800 overflow-hidden" onContextMenu={(e) => e.preventDefault()}>
      <motion.div
        initial={{ scale: 0, opacity: 0, rotate: -180 }}
        animate={{ scale: 1, opacity: 1, rotate: 0 }}
        transition={{ duration: 2, ease: "easeOut" }}
        className="mb-8 relative"
      >
        <img src="/images/Logo.png" alt="Logo" className="w-64 h-64" />
        <motion.div
          className="absolute inset-0 border-4 border-emerald-300 rounded-full"
          initial={{ pathLength: 0 }}
          animate={{ pathLength: 1 }}
          transition={{ duration: 2, ease: "easeInOut" }}
          style={{
            strokeDasharray: "1 1",
            strokeDashoffset: 1,
          }}
        />
      </motion.div>
      <motion.h1
        initial={{ opacity: 0, y: 50 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.5, duration: 1, type: "spring", stiffness: 120 }}
        className="text-emerald-300 text-6xl font-bold text-center mb-4"
      >
        Welcome to Centichain
      </motion.h1>
      <motion.p
        initial={{ opacity: 0, y: 50 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 1, duration: 1, type: "spring", stiffness: 120 }}
        className="text-gray-300 text-3xl mb-8 text-center"
      >
        Empowering the decentralized network revolution
      </motion.p>
      <motion.div
        initial={{ width: 0 }}
        animate={{ width: "80%" }}
        transition={{ duration: 5, ease: "linear" }}
        className="bg-emerald-300 h-2 rounded-full"
      />
      <motion.p
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 1.5, duration: 0.5 }}
        className="text-gray-400 mt-4"
      >
        Loading... {progress}%
      </motion.p>
    </div>
  );
}

export default SplashScreen;
