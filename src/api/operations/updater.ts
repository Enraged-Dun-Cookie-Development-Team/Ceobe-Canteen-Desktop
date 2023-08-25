class Updater {
  async judgmentVersion(version: string, VERSION: string) {
    return true;
  }
}

const updater = new Updater();
export default updater;
