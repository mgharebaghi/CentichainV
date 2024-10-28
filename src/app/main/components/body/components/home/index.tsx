import React from "react";
import { Row, Col, Badge } from "antd";
import { MainBtn } from "@/app/components/tools/components/button";
import { BsStackOverflow } from "react-icons/bs";
import { SiBlockchaindotcom } from "react-icons/si";
import { GrTransaction, GrUpdate } from "react-icons/gr";
import { GiTwoCoins, GiReturnArrow } from "react-icons/gi";
import { TiTick } from "react-icons/ti";
import { FaBan } from "react-icons/fa";
import { HiMiniQueueList } from "react-icons/hi2";
import { IoNotificationsOutline } from "react-icons/io5";

export default function Items(props: any) {
  return (
    <Row>
      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Mempool"}
          badge={
            <Badge
              count={props.mempool.length}
              showZero={true}
              color={props.mempool.length > 0 ? "#ff5f5f" : "#1e3239"}
            />
          }
          icon={<BsStackOverflow size={20} />}
          loading={props.mempoolLoading}
          onClick={props.goToMempool}
        />
      </Col>

      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Blocks"}
          badge={
            <Badge
              count={props.newBlock}
              showZero={true}
              color={props.newBlock > 0 ? "#86a9ff" : "#1e3239"}
            />
          }
          icon={<SiBlockchaindotcom size={20} />}
          loading={props.blockLoading}
          onClick={props.goToBlocks}
        />
      </Col>

      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Transaction"}
          badge={<GrTransaction size={20} />}
          icon={<GiTwoCoins size={25} />}
          loading={props.trxLoading}
          onClick={props.goToTrx}
        />
      </Col>

      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Turn"}
          badge={
            props.turn === "true" ? (
              <TiTick color="#70be00" size={25} />
            ) : (
              <FaBan color="red" size={20} />
            )
          }
          icon={<GiReturnArrow size={20} />}
        />
      </Col>
      
      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Patience"}
          badge={<Badge count={props.patience} showZero={true} color={props.patience === 0 ? "#a6d069" : "#1e3239"} />}
          icon={<HiMiniQueueList size={20} />}
        />
      </Col>

      <Col xs={24} sm={12} md={8} lg={8} xl={8} className="p-2 grid justify-center content-center">
        <MainBtn
          text={"Update"}
          badge={
            <Badge>
              <IoNotificationsOutline color="white" size={20} />
            </Badge>
          }
          icon={<GrUpdate size={20} />}
          onClick={props.goToUpdate}
        />
      </Col>
    </Row>
  );
}
