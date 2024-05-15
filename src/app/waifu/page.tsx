'use client';

// import { appDataDir } from '@tauri-apps/api/path';
// import { readDir } from '@tauri-apps/api/fs';
// import { convertFileSrc } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';

type OutputData = {
	id: number;
	name: string;
	is_last: boolean;
};

export default function Waifu() {

	function handleClick() {
		const getImages = async () => {
			// console.log('base dir', BaseDirectory.AppCache);
			// const images_in_cache = await readDir('images',
			// 	{ dir: BaseDirectory.AppCache, recursive: false }
			// );
			// console.log('images in cache obj', images_in_cache);
			// images_in_cache.forEach(async (entry, i) => {
			// 	console.log(i + ' ' + { entry });
			// });

			if (window.__TAURI__) {
				const {
					appDataDir,
					appCacheDir,
					resourceDir,
					configDir,
					appDir,
					cacheDir,
					dataDir,
					localDataDir,
				} = await import('@tauri-apps/api/path');
				const appDataDirPath = await appDataDir();
				const appCacheDirPath = await appCacheDir();
				const resourceDirPath = await resourceDir();
				const configDirPath = await configDir();
				const appDirPath = await appDir();
				const cacheDirPath = await cacheDir();
				const dataDirPath = await dataDir();
				const localDataDirPath = await localDataDir();
				console.log('appdataDirPath', appDataDirPath);
				console.log('appCacheDirPath', appCacheDirPath);
				console.log('resourceDirPath', resourceDirPath);
				console.log('configDirPath', configDirPath);
				console.log('appDirPath', appDirPath);
				console.log('cacheDirPath', cacheDirPath);
				console.log('dataDirPath', dataDirPath);
				console.log('localDataDirPath', localDataDirPath);
				// appdataDirPath 	C: \Users\codec\AppData\Roaming\com.tauri.dev\
				// appCacheDirPath 	C: \Users\codec\AppData\Local\com.tauri.dev\
				// resourceDirPath 	C: \Users\codec\Documents\embed\tauri - tests\src - tauri\target\debug\
				// configDirPath 	C: \Users\codec\AppData\Roaming\
				// appDirPath 		C: \Users\codec\AppData\Roaming\com.tauri.dev\
				// cacheDirPath 	C: \Users\codec\AppData\Local\
				// dataDirPath 		C: \Users\codec\AppData\Roaming\
				// localDataDirPath C: \Users\codec\AppData\Local\
			}
		}
		getImages();
	}

	return (
		<>
			<h1>Waifu page</h1>
			<button onClick={handleClick} className="bg-green-300 rounded-s" >Run</button>
		</>
	)
}
