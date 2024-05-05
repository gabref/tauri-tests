import { Inter } from "next/font/google";
import "./globals.css";
import dynamic from 'next/dynamic'

const inter = Inter({ subsets: ["latin"] });
import TitleBar from "@/components/titlebar";


export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<body className={inter.className}>
				<TitleBar />
				<main>{children}</main>
			</body>
		</html>
	);
}
