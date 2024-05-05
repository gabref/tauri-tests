import styles from "./titlebar.module.css"
import { appWindow } from "@tauri-apps/api/window"

export function TitleBar() {
	return (
			<div data-tauri-drag-region className={styles.titlebar}>
				<div onClick={() => appWindow.minimize()} className={styles.titlebarButton} id="titlebar-minimize">
					<img
						src="https://api.iconify.design/mdi:window-minimize.svg"
						alt="minimize"
					/>
				</div>
				<div onClick={() => appWindow.toggleMaximize()} className={styles.titlebarButton} id="titlebar-maximize">
					<img
						src="https://api.iconify.design/mdi:window-maximize.svg"
						alt="maximize"
					/>
				</div>
				<div onClick={() => appWindow.close()} className={styles.titlebarButton} id="titlebar-close">
					<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
				</div>
			</div>
	)
}
