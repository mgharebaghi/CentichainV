'use client'
import "./globals.css";
import { CssBaseline } from "@mui/material";
import { useEffect } from "react";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useEffect(() => {
    const disableKeys = (e: KeyboardEvent) => {
      if (e.altKey || e.ctrlKey || (e.key && e.key.startsWith('F') && e.key.length > 1)) {
        e.preventDefault();
      }
    };

    window.addEventListener('keydown', disableKeys);

    return () => {
      window.removeEventListener('keydown', disableKeys);
    };
  }, []);

  return (
    <html lang="en">
      <body className="bg-slate-800 text-slate-200">
        <CssBaseline />
        {children}
      </body>
    </html>
  );
}
