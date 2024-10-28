"use client";
import { Typography, Alert } from "@mui/material";
import { useSearchParams } from "next/navigation";
import { useEffect, useState, Suspense } from "react";
import AOS from "aos";
import { motion } from "framer-motion";
import dynamic from "next/dynamic";

const Syncing = dynamic(() => import("./components/sincyng"), { ssr: false });
const SyncLoader = dynamic(() => import("react-spinners").then(mod => mod.SyncLoader), { ssr: false });

function StarterContent() {
  const params = useSearchParams();
  const [privateKey, setPrivateKey] = useState("");
  const [publicKey, setPublicKey] = useState("");
  const [loading, setLoading] = useState(false);
  const [err, setErr] = useState("");
  const [startVisible, setStartVisible] = useState(true);
  const [status, setStatus] = useState("");
  const [relay, setRelay] = useState("");
  const [turn, setTurn] = useState("");
  const [turnGot, setTurnGot] = useState(false);
  const [dlPercent, setDlPercent] = useState(0);
  const [peerId, setPeerId] = useState("");
  const text = [
    "To start as a validator on the Centichain network, simply click the 'START' button.",
    "This will connect you to a Relay node, synchronize with the network, and join the network as a validator all at once.",
    "For more details, visit Centichain.org."
  ];

  useEffect(() => {
    AOS.init();
    const privateK = params.get("private");
    const publicK = params.get("public");
    setPrivateKey((privateK ?? "").toString());
    setPublicKey((publicK ?? "").toString());

    import("@tauri-apps/api/event").then(async ({ listen }) => {
      await listen<number>("DlPercent", (event) => setDlPercent(event.payload));
      await listen<string>("mongodb", (event) => {
        if (event.payload === "installed") {
          import("@tauri-apps/api/core").then(async ({ invoke }) => {
            await invoke("start", { private: privateK, public: publicK });
          });
        } else if (event.payload === "downloaded") {
          setStatus("Installing MongoDB...");
        } else {
          setErr(event.payload);
        }
      });
      await listen<string>("status", (event) => setStatus(event.payload));
      await listen<string>("relay", (event) => setRelay(event.payload));
      await listen<string>("error", (event) => setErr(event.payload));
      await listen<string>("peerid", (event) => setPeerId(event.payload));
      await listen<string>("turn", (event) => {
        setTurn(event.payload);
        setTurnGot(true);
      });
    });
  }, [params]);

  const start = () => {
    setLoading(true);
    setStartVisible(false);
    import("@tauri-apps/api/core").then(({ invoke }) => {
      invoke("mongodb_download");
      setStatus("Downloading MongoDB...");
    });
  };

  return (
    <div className="flex flex-col items-center justify-center w-screen h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white p-6" onContextMenu={(e) => e.preventDefault()}>
      {startVisible ? (
        <motion.div 
          initial={{ opacity: 0 }} 
          animate={{ opacity: 1 }} 
          transition={{ duration: 0.5 }}
          className="max-w-md text-center mb-8"
        >
          <Typography variant="h4" className="text-emerald-300 mb-6 font-bold">
            Start as Validator
          </Typography>
          <div>
            {text.map((line, i) => (
              <motion.p
                key={i}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.2 }}
                className="text-gray-300 text-lg leading-relaxed"
              >
                {line}
              </motion.p>
            ))}
          </div>
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ type: "spring", stiffness: 260, damping: 20 }}
            className="mt-12"
          >
            <motion.button
              whileHover={{ scale: 1.05, backgroundColor: "#34D399" }}
              whileTap={{ scale: 0.95 }}
              transition={{ type: "spring", stiffness: 400, damping: 10 }}
              className="px-10 py-4 bg-emerald-500 text-white rounded-full shadow-lg"
              onClick={start}
            >
              START
            </motion.button>
          </motion.div>
        </motion.div>
      ) : (
        <Syncing
          privateKey={privateKey}
          publicKey={publicKey}
          relay={relay}
          status={status}
          err={err}
          loading={loading}
          turn={turn}
          turnGot={turnGot}
          dlPercent={dlPercent}
          setTurn={setTurn}
          setTurnGot={setTurnGot}
          peerId={peerId}
        />
      )}
      {err && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="mt-8"
        >
          <Alert severity="error" className="bg-red-900 text-white">
            {err}
          </Alert>
        </motion.div>
      )}
    </div>
  );
}

export default function Starter() {
  return (
    <Suspense fallback={
      <div className="flex flex-col items-center justify-center w-screen h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white">
        <SyncLoader color="#10B981" size={8} />
      </div>
    }>
      <StarterContent />
    </Suspense>
  );
}
