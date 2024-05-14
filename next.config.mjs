/** @type {import('next').NextConfig} */
const nextConfig = {
	output: 'export',
	images: {
		unoptimized: true
	},
	transpilePackages: ['@tauri-apps/api'],
};

export default nextConfig;
