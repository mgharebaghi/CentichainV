"use client";
import React, { useEffect, useState } from 'react';
import { ExitBtn } from "@/app/components/tools/components/button";
import { Typography, Box } from "@mui/material";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Col, Modal, Row } from "antd";
import { BiSolidCoin, BiSolidCoinStack } from "react-icons/bi";
import { IoWalletOutline } from "react-icons/io5";
import { SiRelay } from "react-icons/si";
import { FaExclamationTriangle, FaFingerprint, FaQuestionCircle, FaSignOutAlt } from "react-icons/fa";
import { SyncLoader } from "react-spinners";
import { motion } from "framer-motion";

export default function Header(props: any) {
  const [centies, setCenties] = useState("0.0");
  const [loading, setLoading] = useState(false);
  const [showExitAlert, setShowExitAlert] = useState(false);

  useEffect(() => {
    setLoading(true);
    listen("sum_centies", (event) => {
      setCenties(event.payload as string);
    });
  }, []);

  useEffect(() => {
    invoke<string>("sum_centies", { wallet: props.publicKey }).then(
      (result) => {
        setCenties(result);
        setLoading(false);
      }
    );
  }, [props.publicKey]);

  const handleExitClick = () => {
    setShowExitAlert(true);
  };

  const handleExitConfirm = () => {
    invoke("exit");
  };

  const handleExitCancel = () => {
    setShowExitAlert(false);
  };

  return (
    <Box className="pt-3 pb-1 px-4 bg-gradient-to-r from-slate-800 to-slate-700 rounded-lg shadow-lg">
      <Row align="middle" justify="space-between">
        <Col span={20}>
          <Row gutter={[0, 8]}>
            <Col span={24}>
              <Typography variant="caption" className="text-slate-300 flex items-center">
                <FaFingerprint size={14} className="mr-1 text-slate-400" />
                <span className="text-xs">PeerID: </span>
                <span className="ml-1 text-xs text-slate-300">{props.peerId}</span>
              </Typography>
            </Col>
            <Col span={24}>
              <Typography variant="subtitle2" className="text-slate-300 flex items-center">
                <SiRelay size={16} className="mr-1 text-slate-400" />
                <span className="text-xs">Relay: </span>
                <span className="ml-1 text-xs text-slate-300">{props.relay}</span>
              </Typography>
            </Col>
            <Col span={24}>
              <Typography variant="caption" className="text-slate-300 flex items-center">
                <IoWalletOutline size={14} className="mr-1 text-green-300" />
                <span className="text-xs">Wallet: </span>
                <span className="ml-1 text-xs text-green-300">{props.publicKey}</span>
              </Typography>
            </Col>
            <Col span={24}>
              <Typography variant="subtitle1" className="text-slate-300 flex items-center">
                <BiSolidCoinStack size={18} className="mr-1 text-orange-300" />
                <span className="text-sm">CENTIs: </span>
                {loading ? (
                  <SyncLoader color="#9CA3AF" size={6} className="ml-1" />
                ) : (
                  <motion.div
                    key={centies}
                    initial={{ opacity: 0, y: -50, scale: 0.8 }}
                    animate={{ 
                      opacity: 1, 
                      y: 0, 
                      scale: 1,
                      transition: {
                        type: "spring",
                        stiffness: 200,
                        damping: 15,
                        duration: 0.8
                      }
                    }}
                    className="ml-1 text-sm text-orange-300 font-semibold flex items-center"
                  >
                    <motion.span
                      initial={{ y: -20, opacity: 0 }}
                      animate={{ y: 0, opacity: 1 }}
                      transition={{ delay: 0.2, duration: 0.5 }}
                    >
                      {centies}
                    </motion.span>
                    <motion.div
                      initial={{ y: -50, opacity: 0, rotate: -180 }}
                      animate={{ 
                        y: 0, 
                        opacity: 1, 
                        rotate: 0,
                        transition: {
                          type: "spring",
                          stiffness: 100,
                          damping: 10,
                          delay: 0.3
                        }
                      }}
                      className="ml-1"
                    >
                      <BiSolidCoin className="text-orange-300" />
                    </motion.div>
                  </motion.div>
                )}
              </Typography>
            </Col>
          </Row>
        </Col>
        <Col span={4} className="flex justify-end">
          <ExitBtn onClick={handleExitClick} />
        </Col>
      </Row>

      <Modal
        title={
          <span className="flex items-center text-red-500">
            <FaExclamationTriangle className="mr-2" />
            Confirm Exit
          </span>
        }
        open={showExitAlert}
        onOk={handleExitConfirm}
        onCancel={handleExitCancel}
        okText="Exit"
        cancelText="Cancel"
        okButtonProps={{ 
          danger: true,
          icon: <FaSignOutAlt className="mr-2" />
        }}

      >
        <p className="flex items-center text-gray-600">
          <FaQuestionCircle className="mr-2 text-yellow-500" />
          Are you sure you want to exit? This action cannot be undone.
        </p>
      </Modal>
    </Box>
  );
}
