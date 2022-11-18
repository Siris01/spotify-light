import { BaseDirectory, readTextFile } from '@tauri-apps/api/fs';

export default async function getToken() {
	return readTextFile(BaseDirectory.App + '/token.txt').catch(() => null);
}
