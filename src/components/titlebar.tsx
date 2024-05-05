'use client'

import { useEffect, useState } from "react";
import styles from "./titlebar.module.css"
import { WebviewWindow } from "@tauri-apps/api/window";
import { app } from "@tauri-apps/api";

export default function TitleBar() {
	const [appWindow, setAppWindow] = useState<WebviewWindow | null>(null);

	useEffect(() => {
		const getAppWindow = async () => {
			const appWindow = await import('@tauri-apps/api/window').then(
				lib => lib.appWindow
			).catch(e => console.log('got error' + e));
			if (appWindow)
				setAppWindow(appWindow);
		}
		getAppWindow();
	}, []);

	async function handleMinimize() {
		if (!appWindow)
			return;
		appWindow.minimize();
	}
	async function handleToggleMaximize() {
		if (!appWindow)
			return;
		appWindow.toggleMaximize();
	}
	async function handleClose() {
		if (!appWindow)
			return;
		appWindow.hide();
	}
	return (
		<div data-tauri-drag-region className={styles.titlebar}>
			<div onClick={handleMinimize} className={styles.titlebarButton} id="titlebar-minimize">
				<img
					src="https://api.iconify.design/mdi:window-minimize.svg"
					alt="minimize"
				/>
			</div>
			<div onClick={handleToggleMaximize} className={styles.titlebarButton} id="titlebar-maximize">
				<img
					src="https://api.iconify.design/mdi:window-maximize.svg"
					alt="maximize"
				/>
			</div>
			<div onClick={handleClose} className={styles.titlebarButton} id="titlebar-close">
				<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
			</div>
		</div>
	)
}
