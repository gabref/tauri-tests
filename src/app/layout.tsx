import { Inter } from "next/font/google";
import "./globals.css";
import dynamic from 'next/dynamic'

const inter = Inter({ subsets: ["latin"] });
// https://medium.com/@emdadulislam162/resolving-window-is-not-defined-error-during-npm-run-build-in-next-js-ssr-application-67e2c6197425
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
