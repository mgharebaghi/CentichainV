import React from 'react';
import { Box, Typography } from '@mui/material';
import { Row, Col } from 'antd';
import moment from 'moment';

interface ItemProps {
  index: number;
  item: {
    hash: string;
    date: string;
    value: number;
  };
  from: string;
  to: string;
}

export default function Items({ index, item, from, to }: ItemProps) {
  return (
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
        {/* Display transaction hash */}
        <Col flex="auto" className="h-[100%] grid">
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
            {item.hash.length > 10
              ? `${item.hash.substring(0, 10)}...`
              : item.hash}
          </Typography>
        </Col>
        {/* Display 'from' address */}
        <Col flex="auto" className="h-[100%] grid">
          <Typography
            variant="caption"
            color="grey.500"
            component="span"
            mr={1}
          >
            From:
          </Typography>
          <Typography
            variant="body2"
            noWrap
            color="grey.300"
            component="span"
            sx={{ maxWidth: "100px", display: "inline-block" }}
          >
            {from.length > 10 ? `${from.substring(0, 10)}...` : from}
          </Typography>
        </Col>
        {/* Display 'to' address */}
        <Col flex="auto" className="h-[100%] grid">
          <Typography
            variant="caption"
            color="grey.500"
            component="span"
            mr={1}
          >
            To:
          </Typography>
          <Typography
            variant="body2"
            noWrap
            color="grey.300"
            component="span"
            sx={{ maxWidth: "100px", display: "inline-block" }}
          >
            {to.length > 10 ? `${to.substring(0, 10)}...` : to}
          </Typography>
        </Col>
        {/* Display transaction age */}
        <Col flex="auto" className="h-[100%] grid">
          <Typography
            variant="caption"
            color="grey.500"
            component="span"
            mr={1}
          >
            Age:
          </Typography>
          <Typography variant="body2" color="grey.300" component="span">
            {moment(item.date).fromNow()}
          </Typography>
        </Col>
        {/* Display transaction value */}
        <Col flex="none" className="h-[100%] grid">
          <Typography
            variant="caption"
            color="grey.500"
            component="span"
            mr={1}
          >
            Value:
          </Typography>
          <Typography variant="body2" color="grey.300" component="span">
            {item.value} CENTI
          </Typography>
        </Col>
      </Row>
    </Box>
  );
}
