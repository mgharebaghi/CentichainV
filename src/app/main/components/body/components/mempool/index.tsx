"use client";
import { Col, Row } from "antd";
import { Transaction } from "../..";
import { BackBtn } from "@/app/components/tools/components/button";
import { Typography, Box } from "@mui/material";
import { Pagination } from "@mui/material";
import { useState } from "react";
import { BsDatabaseSlash } from "react-icons/bs";
import Items from "./components/items";

// Define the structure of an Unspent transaction output
interface Unspent {
  hash: string;
  data: any;
}

// Main component for displaying the Mempool
export default function MempoolComponent(props: {
  mempool: Transaction[];
  setShow: any;
  setLoading: any;
}) {
  // State for pagination
  const [page, setPage] = useState(1);
  const transactionsPerPage = 5;

  // Handler for the back button
  const back = () => {
    props.setShow(false);
    props.setLoading(false);
  };

  // Handler for page change in pagination
  const handlePageChange = (
    event: React.ChangeEvent<unknown>,
    value: number
  ) => {
    setPage(value);
  };

  // Calculate the range of transactions to display on the current page
  const indexOfLastTransaction = page * transactionsPerPage;
  const indexOfFirstTransaction = indexOfLastTransaction - transactionsPerPage;
  const currentTransactions = props.mempool.slice(
    indexOfFirstTransaction,
    indexOfLastTransaction
  );

  return (
    <div className="w-full p-2 overflow-hidden">
      {/* Header with back button and Mempool title */}
      <Row className="h-[50px]">
        <Col span={11}>
          <div className="w-[40px] h-[40px] rounded-md">
            <BackBtn onClick={back} />
          </div>
        </Col>

        <Col span={13}>
          <div className="w-full flex items-center justify-center bg-gradient-to-r from-slate-800 to-slate-900 rounded-lg rounded-br-none p-1 shadow-lg">
            <Typography
              variant="h4"
              className="text-white font-bold tracking-wide"
            >
              Mempool
            </Typography>
            <span className="ml-2 bg-slate-700 text-green-300 rounded-full px-2 py-1 text-xs font-semibold">
              {props.mempool.length} Transactions
            </span>
          </div>
        </Col>
      </Row>

      {/* Display transactions or empty state */}
      <Row gutter={[0, 16]}>
        <Col span={24}>
          {props.mempool.length > 0 ? (
            currentTransactions.map((item, index) => {
              // Extract 'from' address from the transaction
              const from = item.signature[0].key;
              // Extract 'to' address(es) from the transaction
              const to = item.output.unspents.map((unspent: Unspent) => {
                if (unspent.data.wallet !== from) {
                  return unspent.data.wallet;
                }
              });

              // Render each transaction
              return (
                <Items key={index} index={index} item={item} from={from} to={to} />
              );
            })
          ) : (
            <div className="w-full h-[300px] flex flex-col items-center justify-center bg-gradient-to-b from-slate-800 to-slate-900 rounded-lg shadow-lg">
              <BsDatabaseSlash size={60} className="text-gray-400 mb-4" />
              <Typography variant="h5" className="text-gray-300 mb-2">
                Mempool is Empty
              </Typography>
              <Typography variant="body2" className="text-gray-500 text-center max-w-md">
                There are currently no pending transactions in the mempool. New transactions will appear here before they are included in a block.
              </Typography>
            </div>
          )}
        </Col>
      </Row>
      {/* Pagination */}
      {props.mempool.length > transactionsPerPage && (
        <Box sx={{ display: "flex", justifyContent: "center", mt: 2 }}>
          <Pagination
            count={Math.ceil(props.mempool.length / transactionsPerPage)}
            page={page}
            onChange={handlePageChange}
            sx={{
              "& .MuiPaginationItem-root": {
                color: "grey.300",
                borderColor: "transparent",
              },
              "& .Mui-selected": {
                backgroundColor: "rgba(255, 255, 255, 0.08)",
                color: "common.white",
              },
              "& .MuiPaginationItem-root:hover": {
                backgroundColor: "rgba(255, 255, 255, 0.12)",
                borderColor: "transparent",
              },
            }}
          />
        </Box>
      )}
    </div>
  );
}
