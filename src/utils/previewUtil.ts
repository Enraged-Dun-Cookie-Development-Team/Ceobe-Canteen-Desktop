import { invoke } from "@tauri-apps/api";

export async function previewUrl(url: string, title: string) {
  await invoke("init_preview", {
    url: url,
    title: title,
  });
}
