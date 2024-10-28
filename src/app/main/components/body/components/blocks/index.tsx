"use client";
import {
  Box,
  IconButton,
  Pagination,
  Typography,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { Col, Row } from "antd";
import { useEffect, useState } from "react";
import { BackBtn } from "@/app/components/tools/components/button";
import moment from "moment";
import { SyncLoader } from "react-spinners";
import { BiDetail } from "react-icons/bi";
import Details from "./components";

// Define the Block interface
export interface Block {
  header: any;
  body: {
    coinbase: any;
    transactions: any[];
  };
}

// // Define the Signature interface
// interface Signature {
//   // Add properties here based on the structure of your signature data
// }

export default function Blocks(props: any) {
  const [page, setPage] = useState(1);
  const [count, setCount] = useState(0);
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [loading, setLoading] = useState(false);
  const [blocksNumber, setBlocksNumber] = useState(0);
  const [selectedBlock, setSelectedBlock] = useState<Block | null>(null);
  const [openDialog, setOpenDialog] = useState(false);

  useEffect(() => {
    setLoading(true);
    invoke<Block[]>("latest_blocks", { page }).then((result) => {
      setBlocks([]);
      setLoading(false);

      if (result.length > 0) {
        setBlocks(result);
        if (page === 1) {
          setCount(Math.ceil(Number(result[0].header.number) / 4));
          setBlocksNumber(result[0].header.number);
        }
      }
      setLoading(false);
    });
  }, [page]);

  const back = () => {
    props.setLoading(false);
    props.setShow(false);
  };

  const handlePageChange = (
    event: React.ChangeEvent<unknown>,
    newPage: number
  ) => {
    setPage(newPage);
  };

  const handleBlockDetails = (block: Block) => {
    setSelectedBlock(block);
    setOpenDialog(true);
  };

  const handleCloseDialog = () => {
    setOpenDialog(false);
  };

  return (
    <div
      onContextMenu={(event) => event.preventDefault()}
      className="w-full p-2 overflow-hidden"
    >
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
              Blockchain
            </Typography>
            <span className="ml-2 bg-slate-700 text-green-300 rounded-full px-2 py-1 text-xs font-semibold">
              {blocksNumber} Blocks
            </span>
          </div>
        </Col>
      </Row>

      {loading ? (
        <div className="w-full min-h-[300px] flex items-center justify-center">
          <SyncLoader color="gray" size={5} />
        </div>
      ) : (
        <div>
          <Row gutter={[0, 16]}>
            <Col span={24}>
              {blocks.map((block, index) => (
                <Box
                  key={index}
                  sx={{
                    border: "1px solid",
                    borderColor: "grey.700",
                    borderRadius: 1,
                    p: 1,
                    mb: 1,
                    backgroundColor: "grey.900",
                  }}
                >
                  <Row gutter={[8, 0]} align="middle" wrap={false}>
                    <Col span={3} className="h-[100%] grid">
                      <Typography
                        variant="caption"
                        color="grey.500"
                        component="span"
                        mr={1}
                      >
                        Number:
                      </Typography>
                      <Typography
                        variant="body2"
                        color="grey.300"
                        component="span"
                      >
                        {block.header.number}
                      </Typography>
                    </Col>
                    <Col span={8} className="h-[100%] grid">
                      <Typography
                        variant="caption"
                        color="grey.500"
                        component="span"
                        mr={1}
                      >
                        Hash:
                      </Typography>
                      <Typography
                        variant="body2"
                        noWrap
                        color="grey.300"
                        component="span"
                        sx={{ maxWidth: "100px", display: "inline-block" }}
                      >
                        {block.header.hash.length > 25
                          ? `${block.header.hash.substring(0, 25)}...`
                          : block.header.hash}
                      </Typography>
                    </Col>
                    <Col span={5} className="h-[100%] grid">
                      <Typography
                        variant="caption"
                        color="grey.500"
                        component="span"
                        mr={1}
                      >
                        Transactions:
                      </Typography>
                      <Typography
                        variant="body2"
                        color="grey.300"
                        component="span"
                      >
                        {block.body.transactions.length}
                      </Typography>
                    </Col>
                    <Col span={6} className="h-[100%] grid">
                      <Typography
                        variant="caption"
                        color="grey.500"
                        component="span"
                        mr={1}
                      >
                        Time:
                      </Typography>
                      <Typography
                        variant="body2"
                        color="grey.300"
                        component="span"
                      >
                        {moment(block.header.date).fromNow()}
                      </Typography>
                    </Col>
                    <Col span={2}>
                      <IconButton
                        onClick={() => handleBlockDetails(block)}
                        sx={{ color: 'grey.500' }}
                      >
                        <BiDetail />
                      </IconButton>
                    </Col>
                  </Row>
                </Box>
              ))}
            </Col>
          </Row>

          <Row justify="center">
            <Pagination
              count={count}
              page={page}
              onChange={handlePageChange}
              variant="outlined"
              shape="rounded"
              sx={{
                '& .MuiPaginationItem-root': {
                  color: 'grey.300',
                  borderColor: 'transparent',
                },
                '& .Mui-selected': {
                  backgroundColor: 'rgba(255, 255, 255, 0.08)',
                  color: 'common.white',
                },
                '& .MuiPaginationItem-root:hover': {
                  backgroundColor: 'rgba(255, 255, 255, 0.12)',
                  borderColor: 'transparent',
                },
                '& .MuiPaginationItem-ellipsis': {
                  display: 'none',
                },
              }}
            />
          </Row>
        </div>
      )}

      <Details block={selectedBlock} openDialog={openDialog} handleCloseDialog={handleCloseDialog}/>
    </div>
  );
}
