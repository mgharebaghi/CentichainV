"use client";
import { Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { Alert, Col, Input, Row, Modal } from "antd";
import { useEffect, useState } from "react";
import { SyncLoader } from "react-spinners";
import { readText } from "@tauri-apps/plugin-clipboard-manager";
import {
  BackBtn,
  CentiBtn,
  PasteBtn,
} from "@/app/components/tools/components/button";
import { motion } from "framer-motion";
import { FaCoins, FaUser } from "react-icons/fa";

interface Response {
  status: string;
  description: string;
}

export default function Trx(props: any) {
  const [to, setTo] = useState("");
  const [value, setValue] = useState("");
  const [status, setStatus] = useState<Response>();
  const [statusLoading, setStatusLoading] = useState(false);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [centies, setCenties] = useState("0.0");
  const [fee, setFee] = useState("0");

  useEffect(() => {
    invoke<string>("sum_centies", { wallet: props.publicKey }).then(
      (result) => {
        setCenties(result);
      }
    );
  }, []);

  useEffect(() => {
    if (value) {
      const feeValue = (parseFloat(value) * 0.01).toFixed(2);
      setFee(feeValue);
    } else {
      setFee("0");
    }
  }, [value]);

  const sendBtn = () => {
    if (to === "") {
      setStatus({
        status: "error",
        description:
          "Wallet address is required! Please enter wallet address to proceed.",
      });
    } else if (to === props.publicKey) {
      setStatus({
        status: "error",
        description: "Wallet address must be different from yours!",
      });
    } else if (value === "" || value === "0") {
      setStatus({
        status: "error",
        description: "Value is incorrect.",
      });
    } else if (value.startsWith("0") && value[1] !== ".") {
      setStatus({
        status: "error",
        description: "Invalid value format. If starting with 0, it must be followed by a decimal point.",
      });
    } else if (parseFloat(centies) < parseFloat(value) + parseFloat(fee)) {
      setStatus({
        status: "error",
        description:
          "Insufficient funds. Please check your balance including the 1% fee.",
      });
    } else {
      setStatus(undefined);
      setIsModalVisible(true);
    }
  };

  const handleConfirm = () => {
    setIsModalVisible(false);
    setStatusLoading(true);
    invoke<Response>("send_transaction", {
      wallet: props.publicKey,
      private: props.privateKey,
      to: to,
      value: value,
    }).then((result) => {
      setStatusLoading(false);
      setStatus(result);
    });
  };

  const back = () => {
    props.setLoading(false);
    props.setShow(false);
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
      className="w-full p-2 bg-gradient-to-br from-gray-900 to-gray-800 text-white rounded-lg shadow-lg"
      onContextMenu={(e) => e.preventDefault()}
    >
      <Row className="h-[50px] mb-4">
        <Col span={11}>
          <div className="w-[40px] h-[40px] rounded-md">
            <BackBtn onClick={back} />
          </div>
        </Col>

        <Col span={13}>
          <div className="w-full flex items-center justify-center bg-gradient-to-r from-slate-800 to-slate-900 rounded-lg rounded-br-none p-1 shadow-lg">
            <Typography
              variant="h4"
              className="text-emerald-300 font-bold tracking-wide"
            >
              Make Transaction
            </Typography>
          </div>
        </Col>
      </Row>

      <Row className="h-[35px] flex items-center mb-4">
        <Col span={2}>
          <Typography className="text-gray-300">Wallet:</Typography>
        </Col>
        <Col span={21}>
          <Input
            value={to}
            type="text"
            spellCheck={false}
            onChange={(e) => {
              setStatus({
                status: "",
                description: "",
              });
              setTo(e.target.value);
            }}
            className="h-[35px] border-slate-500 bg-slate-800 focus:bg-slate-700 focus:border-slate-500 hover:bg-slate-800 hover:border-slate-500 text-slate-200 border-r-0 rounded-r-none"
          />
        </Col>
        <Col span={1} className="h-full">
          <PasteBtn
            onClick={async () => {
              const copied = await readText();
              setTo(copied ?? "");
            }}
          />
        </Col>
      </Row>

      <Row className="h-[35px] flex items-center mb-4">
        <Col span={2}>
          <Typography className="text-gray-300">Value:</Typography>
        </Col>
        <Col span={21}>
          <Input
            value={value}
            type="number"
            min={0}
            step="0.01"
            spellCheck={false}
            onChange={(e) => setValue(e.target.value)}
            className="h-[35px] border-slate-500 bg-slate-800 focus:bg-slate-700 focus:border-slate-500 hover:bg-slate-800 hover:border-slate-500 text-slate-200 border-r-0 rounded-r-none"
          />
        </Col>
        <Col span={1} className="h-full">
          <PasteBtn
            onClick={async () => {
              const copied = await readText();
              setValue(copied ?? "");
            }}
          />
        </Col>
      </Row>

      <Row className="mb-4">
        <Col span={24}>
          <Typography className="text-gray-300">
            Fee <span className="text-emerald-400 text-xs">(1%)</span>: {fee}{" "}
            CENTIs
          </Typography>
        </Col>
      </Row>

      <Row className="mb-4">
        <Col span={24} className="grid justify-center">
          <CentiBtn text="Send" onClick={sendBtn} />
        </Col>
      </Row>

      <Row>
        <Col span={24} className="grid justify-center">
          {statusLoading ? (
            <div>
              <SyncLoader size={5} color="#10B981" />
            </div>
          ) : status?.status ? (
            <Alert
              type={status.status === "success" ? "success" : "error"}
              message={status.description}
              className={`${
                status.status === "error"
                  ? "bg-red-900 border-red-700"
                  : "bg-slate-800 border-slate-700"
              } text-white`}
            />
          ) : null}
        </Col>
      </Row>

      <Modal
        title={<span className="text-gray-800">Confirm Transaction</span>}
        open={isModalVisible}
        onOk={handleConfirm}
        onCancel={() => setIsModalVisible(false)}
        okText="Confirm"
        cancelText="Cancel"
        className="bg-white text-gray-800 rounded-lg"
        okButtonProps={{
          className:
            "bg-emerald-500 text-white border-none hover:bg-emerald-600 rounded-md",
        }}
        cancelButtonProps={{
          className:
            "bg-gray-300 text-gray-800 border-none hover:bg-gray-400 rounded-md",
        }}
      >
        <p className="text-gray-600 mb-2">
          Are you sure you want to send this transaction?
        </p>
        <p className="text-gray-600 flex items-center mb-2">
          <FaUser className="mr-2 text-gray-500" />
          <span className="font-bold">To:</span>{" "}
          <span className="ml-2 font-bold">{to}</span>
        </p>
        <p className="text-gray-600 flex items-center mb-2">
          <FaCoins className="mr-2 text-gray-500" />
          <span className="font-bold">Value:</span>{" "}
          <span className="ml-2 font-bold">{value}</span>
        </p>
        <p className="text-gray-600 flex items-center mb-4">
          <FaCoins className="mr-2 text-gray-500" />
          <span className="font-bold">
            Fee <span className="text-gray-500 text-sm">(1%)</span>:
          </span>{" "}
          <span className="ml-2 font-bold">{fee}</span>
        </p>
        <Alert
          message="UTXO Model Warning"
          description="Please note that this transaction follows the UTXO model. If confirmed, it will spend the entire UTXO, potentially resulting in change being sent back to your wallet."
          type="warning"
          showIcon
          className="bg-yellow-50 border-yellow-200 text-yellow-700 rounded-md"
        />
      </Modal>
    </motion.div>
  );
}
