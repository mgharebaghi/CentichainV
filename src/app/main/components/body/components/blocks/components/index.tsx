import {
  Dialog,
  DialogContent,
  DialogActions,
  Button,
  Typography,
  Grid,
  Paper,
  List,
  ListItem,
  ListItemText,
  Box,
} from "@mui/material";

export default function Details(props: any) {
  return (
    <Dialog
      open={props.openDialog}
      onClose={props.handleCloseDialog}
      maxWidth="md"
      fullWidth
      PaperProps={{
        style: {
          backgroundColor: "#121212",
          color: "#FFFFFF",
          borderRadius: "16px",
          userSelect: "none", // Make the dialog content non-selectable
        },
      }}
      sx={{
        "& .MuiDialogContent-root": {
          "&::-webkit-scrollbar": {
            width: "6px",
          },
          "&::-webkit-scrollbar-track": {
            background: "#1E1E1E",
          },
          "&::-webkit-scrollbar-thumb": {
            background: "#333",
            borderRadius: "3px",
          },
          "&::-webkit-scrollbar-thumb:hover": {
            background: "#555",
          },
        },
      }}
    >
      <DialogContent sx={{ padding: "32px", overflowX: "hidden" }}>
        {props.block && (
          <Box>
            {/* Block title */}
            <Typography
              variant="h4"
              color="#00BFFF"
              gutterBottom
              fontWeight="bold"
              sx={{
                textShadow: "2px 2px 4px rgba(0,0,0,0.5)",
                letterSpacing: "1px",
              }}
            >
              Block Details: #{props.block.header.number}
            </Typography>
            <Grid container spacing={3}>
              {/* Header section */}
              <Grid item xs={12}>
                <Paper
                  elevation={0}
                  sx={{
                    p: 3,
                    backgroundColor: "rgba(255, 255, 255, 0.05)",
                    borderRadius: "12px",
                  }}
                >
                  <Typography
                    variant="h5"
                    color="#00BFFF"
                    gutterBottom
                    sx={{
                      borderBottom: "2px solid",
                      paddingBottom: "8px",
                      marginBottom: "16px",
                    }}
                  >
                    Header
                  </Typography>
                  <Grid container spacing={2}>
                    {Object.entries(props.block.header).map(([key, value]) => (
                      <Grid item xs={12} sm={6} md={4} key={key}>
                        <Box>
                          <Typography variant="subtitle2" color="grey.400">
                            {key}
                          </Typography>
                          {key === "signature" &&
                          typeof value === "object" &&
                          value !== null ? (
                            <Box sx={{ mt: 1 }}>
                              {Object.entries(value).map(
                                ([sigKey, sigValue]) => (
                                  <Box key={sigKey} sx={{ mb: 1 }}>
                                    <Typography
                                      variant="caption"
                                      color="grey.500"
                                    >
                                      {sigKey}:
                                    </Typography>
                                    <Typography
                                      variant="body2"
                                      color="white"
                                      sx={{
                                        wordBreak: "break-all",
                                        fontWeight: "medium",
                                      }}
                                    >
                                      {String(sigValue)}
                                    </Typography>
                                  </Box>
                                )
                              )}
                            </Box>
                          ) : (
                            <Typography
                              variant="body1"
                              color="white"
                              sx={{
                                wordBreak: "break-all",
                                fontWeight: "medium",
                              }}
                            >
                              {typeof value === "object" && value !== null
                                ? JSON.stringify(value, null, 2)
                                : String(value)}
                            </Typography>
                          )}
                        </Box>
                      </Grid>
                    ))}
                  </Grid>
                </Paper>
              </Grid>
              {/* Coinbase section */}
              {props.block.body.coinbase && (
                <Grid item xs={12}>
                  <Paper
                    elevation={0}
                    sx={{
                      p: 3,
                      backgroundColor: "rgba(255, 255, 255, 0.05)",
                      borderRadius: "12px",
                    }}
                  >
                    <Typography
                      variant="h5"
                      color="#00BFFF"
                      gutterBottom
                      sx={{
                        borderBottom: "2px solid",
                        paddingBottom: "8px",
                        marginBottom: "16px",
                      }}
                    >
                      Coinbase
                    </Typography>
                    <Grid container spacing={2}>
                      {Object.entries(props.block.body.coinbase).map(
                        ([key, value]) => (
                          <Grid item xs={12} sm={6} md={4} key={key}>
                            <Box>
                              <Typography variant="subtitle2" color="grey.400">
                                {key}
                              </Typography>
                              <Typography
                                variant="body1"
                                color="white"
                                sx={{
                                  wordBreak: "break-all",
                                  fontWeight: "medium",
                                }}
                              >
                                {key === "output" &&
                                typeof value === "object" &&
                                value !== null
                                  ? `Number of outputs: ${
                                      Object.keys(value).length
                                    }`
                                  : typeof value === "object"
                                  ? JSON.stringify(value, null, 2)
                                  : String(value)}
                              </Typography>
                            </Box>
                          </Grid>
                        )
                      )}
                    </Grid>
                  </Paper>
                </Grid>
              )}

              {/* Transactions section */}
              <Grid item xs={12}>
                <Paper
                  elevation={0}
                  sx={{
                    p: 3,
                    backgroundColor: "rgba(255, 255, 255, 0.05)",
                    borderRadius: "12px",
                  }}
                >
                  <Typography
                    variant="h5"
                    color="#00BFFF"
                    gutterBottom
                    sx={{
                      borderBottom: "2px solid",
                      paddingBottom: "8px",
                      marginBottom: "16px",
                    }}
                  >
                    Transactions ({props.block.body.transactions.length})
                  </Typography>
                  {props.block.body.transactions.length > 0 ? (
                    <List>
                      {props.block.body.transactions.map(
                        (tx: { hash?: string }, index: number) => (
                          <ListItem key={index} disableGutters>
                            <ListItemText
                              primary={
                                <Typography
                                  variant="body1"
                                  color="white"
                                  sx={{
                                    wordBreak: "break-all",
                                    fontWeight: "medium",
                                  }}
                                >
                                  {tx.hash || "Hash not available"}
                                </Typography>
                              }
                            />
                          </ListItem>
                        )
                      )}
                    </List>
                  ) : (
                    <Typography variant="body1" color="gray">
                      No transactions in this block.
                    </Typography>
                  )}
                </Paper>
              </Grid>
            </Grid>
          </Box>
        )}
      </DialogContent>
      <DialogActions sx={{ padding: "16px 32px" }}>
        <Button
          onClick={props.handleCloseDialog}
          variant="contained"
          color="primary"
          size="large"
        >
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
}
