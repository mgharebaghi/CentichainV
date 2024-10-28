"use client";
import { listen } from "@tauri-apps/api/event";
import AOS from "aos";
import { useEffect, useState } from "react";
import MempoolComponent from "./components/mempool";
import Blocks from "./components/blocks";
import Items from "./components/home";
import Trx from "./components/transaction";
import Update from "./components/update";

// Define the Transaction interface
export interface Transaction {
  hash: string;
  input: any;
  output: any;
  value: any;
  fee: any;
  script: any;
  signature: any;
  date: any;
}

export default function Body(props: any) {
  // State variables
  const [blockLoading, setBlockLoading] = useState(false);
  const [mempoolLoading, setMempooolLoading] = useState(false);
  const [mempool, setMempool] = useState<Transaction[]>([]);
  const [newBlock, setNewBlock] = useState(0);
  const [trxLoading, setTrxLoading] = useState(false);
  const [showMempool, setShowMempool] = useState(false);
  const [showBlocks, setShowBlocks] = useState(false);
  const [showTrx, setShowTrx] = useState(false);
  const [showUpdate, setShowUpdate] = useState(false);
  const [patience, setPatience] = useState<number>(0);

  useEffect(() => {
    AOS.init();

    // Listen to get mempool from backend as an array
    listen<Transaction[]>("mempool", (event) => {
      setMempool(event.payload);
    });

    // Listen to get new blocks number from backend to change count of new blocks
    listen<string>("block", () => {
      setNewBlock((prevData) => prevData + 1);
    });

    // Listen to show validator turn
    listen<string>("turn", (event) => {
      props.setTurn(event.payload);
    });

    listen<number>("patience", (event) => {
      setPatience(event.payload);
    });
  }, []);

  useEffect(() => {
    props.setTurn(props.turn);
  }, [props.turn]);

  // Navigate to blocks page
  const goToBlocks = () => {
    setBlockLoading(true);
    setShowBlocks(true);
    setNewBlock(0);
  };

  // Navigate to mempool page
  const goToMempool = () => {
    setMempooolLoading(true);
    setShowMempool(true);
  };

  // Navigate to transaction page
  const goToTrx = () => {
    setTrxLoading(true);
    setShowTrx(true);
  };

  const goToUpdate = () => {
    setShowUpdate(true);
  };

  return (
    <div className="w-full overflow-auto">
      {showMempool ? (
        <MempoolComponent
          mempool={mempool}
          setShow={setShowMempool}
          setLoading={setMempooolLoading}
        />
      ) : showBlocks ? (
        <Blocks setShow={setShowBlocks} setLoading={setBlockLoading} />
      ) : showTrx ? (
        <Trx
          publicKey={props.publicKey}
          privateKey={props.privateKey}
          setShow={setShowTrx}
          setLoading={setTrxLoading}
        />
      ) : showUpdate ? (
        <Update setShow={setShowUpdate} />
      ) : (
        <Items
          mempoolLoading={mempoolLoading}
          mempool={mempool}
          newBlock={newBlock}
          setNewBlock={setNewBlock}
          blockLoading={blockLoading}
          goToBlocks={goToBlocks}
          goToMempool={goToMempool}
          goToTrx={goToTrx}
          trxLoading={trxLoading}
          turn={props.turn}
          patience={patience}
          goToUpdate={goToUpdate}
        />
      )}
    </div>
  );
}
