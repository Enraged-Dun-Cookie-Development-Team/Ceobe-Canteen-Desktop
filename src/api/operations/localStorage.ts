class LocalStorage {
  getItem(key: string): string | null {
    return null;
  }
  setItem(key: string, value: string) {}
}

const storage = new LocalStorage();

export default storage;
