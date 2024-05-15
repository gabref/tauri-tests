import { BaseDirectory, createDir, exists, readBinaryFile, writeBinaryFile } from '@tauri-apps/api/fs';
import axios from 'axios';

const CACHE_DIR = "cache";

/**
 * Cache an image from a given URL to the specified cache directory
 * 
 * @param {string} imageUrl - the url of the image to be cached
 * @returns {Promise<void>} A promise that resolves when the image is succesfully cached
 * @throws {Error} if there's an error fetching, creating the cache directory, or writing the image data
 */
async function cacheImage(imageUrl: string): Promise<void> {
	try {
		const imageData = await fetchImageData(imageUrl);
		const imageName = getImageName(imageUrl);
		const imagePath = getImagePath(imageName);

		await createCacheDirectory();
		await writeImageDataToCache(imagePath, imageData);

		console.log("Image cached successfully:", imagePath);
	} catch (error) {
		console.error("Error caching image: ", error);
	}
}

/**
 * Fetch image data from the provided URL
 * 
 * @param {string} imageUrl - the url of the image to be cached
 * @returns {Promise<ArrayBuffer>} A promise that resolves to the fetched image data
 */
async function fetchImageData(imageUrl: string): Promise<ArrayBuffer> {
	try {
		const response = await axios.get(imageUrl, { responseType: "arraybuffer" });
		return response.data;
	} catch (error) {
		throw new Error("Error fetching image data: " + error);
	}
}

/**
 * Extracts the image name from the provided URL
 * 
 * @param {string} imageUrl - the url of the image
 * @returns {string} The extracted image name
 */
function getImageName(imageUrl: string): string {
	return imageUrl.substring(imageUrl.lastIndexOf("/") + 1);
}

/**
 * Generates the full path to the cache directory
 * 
 * @param {string} imageName - The name of the image
 * @returns {string} The full path to the cache directory
 */
function getImagePath(imageName: string): string {
	return `${CACHE_DIR}/${imageName}`;
}

/**
 * Creates the cache directory if it doesn't exist
 * 
 * @returns {Promise<void>} The full path to the cache directory
 */
async function createCacheDirectory() {
	try {
		await createDir(CACHE_DIR, {
			recursive: true,
			dir: BaseDirectory.AppData,
		});
	} catch (error) {
		throw new Error("Error creating cache directory: " + error);
	}
}

/**
 * Writes image data to the cache directory
 * 
 * @param {string} imagePath - The path to the image file
 * @param {ArrayBuffer} imageData - The image data to write
 * @returns {Promise<void>} A promise that resolves when the image data is written to the cache
 */
async function writeImageDataToCache(imagePath: string, imageData: ArrayBuffer): Promise<void> {
	try {
		await writeBinaryFile(imagePath, new Uint8Array(imageData), {
			dir: BaseDirectory.AppData,
		});
	} catch (error) {
		throw new Error("Error writing image data to cache: " + error);
	}
}

/**
 * Display a cached image or cache and display a new image
 * 
 * @param {string} imageUrl - The URL of the image to be displayed or cached
 * @param {Promise<string>} A promise that resolves to a base64-encoded image data URI or the original image URL
 * @throws {Error} If there's an error reading or caching the image
 * @example
 * const imageUrl = "https://example.com/image.jpg";
 * const cachedImage = await displayCachedImage(imageUrl);
 * console.log(cachedImage); // Outputs a base64-encoded image data URI or the original image URL
 */
export async function displayCachedImage(imageUrl: string) {
	const imageName = getImageName(imageUrl);
	const imagePath = getImagePath(imageName);
	const imageExists = await exists(imagePath, {
		dir: BaseDirectory.AppData,
	});

	if (imageExists) {
		// read the binary file
		const u8Array = await readBinaryFile(imagePath, {
			dir: BaseDirectory.AppData,
		});

		console.info("Returned from cache");
		// convert to base64 to consume it in the image tag
		const base64Image = arrayBufferToBase64(u8Array);

		return base64Image;
	} else {
		// cache the image
		cacheImage(imageUrl);
		return imageUrl;
	}
}

/**
 * Converst a Uint8Array to a base64-encoded Data URI
 * 
 * @param {Uint8Array} uint8Array - The Uint8Array to convert to base64.
 * @returns {string} A Data URI in the format "data:image/jpg;base64,<base64String>".
 * @example
 * const byteArray = new Uint8Array([255, 216, 255, 224, 0, 16, 74, 70, ...]);
 * const dataUri = _arrayBufferToBase64(byteArray);
 * console.log(dataUri); // Outputs a base64-encoded Data URI.
 */
function arrayBufferToBase64(uint8Array: Uint8Array): string {
	const decoder = new TextDecoder('utf8');
	const base64String = btoa(decoder.decode(uint8Array));
	// const base64String = btoa(String.fromCharCode(...uint8Array));
	return `data:image/jpg;base64,${base64String}`;
}
