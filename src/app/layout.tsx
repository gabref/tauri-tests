import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });
import TitleBar from "@/components/titlebar";
import { Header } from "@/components/header";


export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<body className={inter.className}>
				<TitleBar />
				<Header />
				<main>{children}</main>
			</body>
		</html>
	);
}
