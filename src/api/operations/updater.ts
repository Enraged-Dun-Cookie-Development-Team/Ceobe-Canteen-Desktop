import { app } from "@tauri-apps/api";

class Updater {
  async judgmentVersion(newer: string) {
    let v1 = await app.getVersion();
    let v2 = newer;
    console.log(v1);
    console.log(v2);
    if (v1 == v2) {
      return false;
    }

    const vs1 = v1.split(".").map((a: string) => parseInt(a));
    const vs2 = v2.split(".").map((a) => parseInt(a));

    const digit = Math.min(vs1.length, vs2.length);
    for (let i = 0; i < digit; i++) {
      if (vs1[i] > vs2[i]) {
        return true;
      } else if (vs1[i] < vs2[i]) {
        return false;
      }
    }

    return digit != vs1.length;
  }
}

const updater = new Updater();
export default updater;
