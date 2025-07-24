/**
 * Gets the path of the blazing-fast-rust binary for the current platform
 *
 * @returns Filesystem path to the binary, or null if no prebuilt distribution exists for the current platform
 */
export function getCommand(): string | null {
	const { platform, arch } = process;

	type PlatformPaths = {
		[P in NodeJS.Platform]?: {
			[A in NodeJS.Architecture]?: string;
		};
	};

	const PLATFORMS: PlatformPaths = {
		win32: {
			x64: "@blazing-fast-rust/cli-win32-x64/blazing-fast-rust.exe",
			arm64: "@blazing-fast-rust/cli-win32-arm64/blazing-fast-rust.exe",
		},
		darwin: {
			x64: "@blazing-fast-rust/cli-darwin-x64/blazing-fast-rust",
			arm64: "@blazing-fast-rust/cli-darwin-arm64/blazing-fast-rust",
		},
		linux: {
			x64: "@blazing-fast-rust/cli-linux-x64/blazing-fast-rust",
			arm64: "@blazing-fast-rust/cli-linux-arm64/blazing-fast-rust",
		},
	};

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
