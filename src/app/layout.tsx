'use client'

import { Inter } from "next/font/google";
import "./globals.css";
import dynamic from 'next/dynamic'

const inter = Inter({ subsets: ["latin"] });
const TitleBar = dynamic(() => import('@/components/titlebar'), { 
	ssr: false 
});

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<TitleBar />
			<body className={inter.className}>{children}</body>
		</html>
	);
}
