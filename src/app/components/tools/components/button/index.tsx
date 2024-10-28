import { Typography } from "@mui/material";
import { Col, Row } from "antd";
import { BiArrowToLeft } from "react-icons/bi";
import { FaPaste } from "react-icons/fa";
import { SyncLoader } from "react-spinners";
import { motion } from 'framer-motion';

//main style button
export function CentiBtn(props: any) {
  return (
    <div
      className="w-[300px] h-[40px] select-none flex justify-center items-center 
      rounded-md cursor-pointer text-center border-[1px] border-slate-500 text-white 
      hover:bg-slate-700 active:bg-slate-800 transition duration-200"
      onClick={props.onClick}
    >
      <Typography>{props.text}</Typography>
    </div>
  );
}

//start button
export function CentiBtnCircul({ onClick, text }: { onClick: () => void; text: string }) {
  return (
    <motion.div
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      className="w-36 h-36 rounded-full flex items-center justify-center cursor-pointer
                 bg-emerald-500 text-white shadow-lg
                 transition-all duration-300 ease-in-out border-2 border-white"
      onClick={onClick}
    >
      <Typography variant="button" className="font-bold text-lg">
        {text}
      </Typography>
    </motion.div>
  );
}

//white button
export function CentiBtnLighter(props: any) {
  return (
    <div
      className="w-[300px] h-[40px] select-none flex justify-center items-center 
      rounded-md cursor-pointer text-center 
      bg-slate-300 text-slate-950 hover:bg-white hover:text-slate-800 active:bg-slate-400 transition duration-200"
      onClick={props.onClick}
    >
      <Typography>{props.text}</Typography>
    </div>
  );
}

//back button
export function BackBtn(props: any) {
  return (
    <div
      onClick={props.onClick}
      className="cursor-pointer border-[1px] rounded-md text-slate-300 w-[40px] h-[40px] border-slate-500 flex justify-center items-center hover:bg-slate-600 hover:text-white transition duration-200"
    >
      <BiArrowToLeft size={25} />
    </div>
  );
}

//exit in main page
export function ExitBtn({ onClick }: { onClick: () => void }) {
  return (
    <motion.div
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      className="rounded-full text-white w-16 h-16 flex justify-center items-center 
                 bg-red-500 hover:bg-red-600 transition-colors duration-200 cursor-pointer 
                 shadow-md"
      onClick={onClick}
    >
      <Typography variant="button" className="font-bold">
        EXIT
      </Typography>
    </motion.div>
  );
}

//main buttons
export function MainBtn(props: any) {
  return (
    <motion.div
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      className="w-[200px] h-[150px] select-none flex justify-center items-center 
      rounded-md cursor-pointer text-center border-[1px] border-slate-600 text-slate-100 
      bg-slate-800 hover:bg-slate-700 active:bg-slate-600 transition duration-200 shadow-md"
      onClick={props.onClick}
    >
      <Row className="h-[100%]">
        <Col span={24} className="h-[60%] grid justify-center content-end">
          <Row>
            <Col
              span={5}
              className="grid justify-items-end content-center pr-2"
            >
              {props.icon}
            </Col>
            <Col span={19} className="grid justify-items-end pr-2">
              <Typography variant="h5" className="text-slate-100">{props.text}</Typography>
            </Col>
          </Row>
        </Col>
        <Col span={24} className="h-[40%] grid justify-center pt-3">
          {props.loading ? (
            <div>
              <SyncLoader size={5} color="#94a3b8" />
            </div>
          ) : (
            props.badge
          )}
        </Col>
      </Row>
    </motion.div>
  );
}

// paste btn
export function PasteBtn(props: any) {
  return (
    <div
    onClick={props.onClick} 
    className="w-full h-full grid content-center justify-center border-[1px] border-slate-500 rounded-md rounded-l-none cursor-pointer transition duration-150 hover:bg-slate-700 active:bg-slate-800">
      <FaPaste />
    </div>
  );
}


